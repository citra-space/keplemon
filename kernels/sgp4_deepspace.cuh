// SGP4 Deep Space Functions (CUDA)
// Based on Vallado's DSCOM, DSINIT, DSPACE, DPPER functions
// Handles satellites with orbital periods > 225 minutes

#ifndef SGP4_DEEPSPACE_CUH
#define SGP4_DEEPSPACE_CUH

#include "sgp4_types.cuh"
#include "sgp4_constants.cuh"

// ═══════════════════════════════════════════════════════════════════════════
// DSCOM - Deep Space Common Items
// Computes lunar-solar constants needed for deep space calculations
// ═══════════════════════════════════════════════════════════════════════════
__device__ void dscom(
    double epoch, double ep, double argpp, double tc, double inclp,
    double nodep, double np,
    // Outputs
    double& snodm, double& cnodm, double& sinim, double& cosim,
    double& sinomm, double& cosomm, double& day, double& emsq,
    double& gam, double& rtemsq, double& s1, double& s2, double& s3,
    double& s4, double& s5, double& s6, double& s7, double& ss1,
    double& ss2, double& ss3, double& ss4, double& ss5, double& ss6,
    double& ss7, double& sz1, double& sz2, double& sz3, double& sz11,
    double& sz12, double& sz13, double& sz21, double& sz22, double& sz23,
    double& sz31, double& sz32, double& sz33, double& z1, double& z2,
    double& z3, double& z11, double& z12, double& z13, double& z21,
    double& z22, double& z23, double& z31, double& z32, double& z33,
    double& nm, double& em, double& inclm, double& mm, double& argpm, double& nodem,
    Sgp4Params& p
) {
    // Solar and lunar constants (from sgp4_constants.cuh)
    // ZES, ZEL, C1SS, C1L, ZSINIS, ZCOSIS, ZCOSGS, ZSINGS
    
    // Local variables
    nm = np;
    em = ep;
    snodm = sin(nodep);
    cnodm = cos(nodep);
    sinomm = sin(argpp);
    cosomm = cos(argpp);
    sinim = sin(inclp);
    cosim = cos(inclp);
    emsq = em * em;
    double betasq = 1.0 - emsq;
    rtemsq = sqrt(betasq);
    
    // Initialize lunar-solar terms
    p.peo = 0.0;
    p.pinco = 0.0;
    p.plo = 0.0;
    p.pgho = 0.0;
    p.pho = 0.0;
    
    // Day calculation (fraction of day since Jan 0, 1950)
    day = epoch + 18261.5 + tc / 1440.0;
    
    double xnodce = fmod(4.5236020 - 9.2422029e-4 * day, TWOPI);
    double stem = sin(xnodce);
    double ctem = cos(xnodce);
    double zcosil = 0.91375164 - 0.03568096 * ctem;
    double zsinil = sqrt(1.0 - zcosil * zcosil);
    double zsinhl = 0.089683511 * stem / zsinil;
    double zcoshl = sqrt(1.0 - zsinhl * zsinhl);
    gam = 5.8351514 + 0.0019443680 * day;
    double zx = ZSINIS * stem / zsinil;
    double zy = zcoshl * ctem + ZCOSIS * zsinhl * stem;
    zx = atan2(zx, zy);
    zx = gam + zx - xnodce;
    double zcosgl = cos(zx);
    double zsingl = sin(zx);
    
    // Solar terms - use constants from header
    double zcosg = ZCOSGS;
    double zsing = ZSINGS;
    double zcosi = ZCOSIS;
    double zsini = ZSINIS;
    double zcosh = cnodm;
    double zsinh = snodm;
    double cc = C1SS;
    double xnoi = 1.0 / nm;
    
    for (int lsflg = 1; lsflg <= 2; lsflg++) {
        double a1 = zcosg * zcosh + zsing * zcosi * zsinh;
        double a3 = -zsing * zcosh + zcosg * zcosi * zsinh;
        double a7 = -zcosg * zsinh + zsing * zcosi * zcosh;
        double a8 = zsing * zsini;
        double a9 = zsing * zsinh + zcosg * zcosi * zcosh;
        double a10 = zcosg * zsini;
        double a2 = cosim * a7 + sinim * a8;
        double a4 = cosim * a9 + sinim * a10;
        double a5 = -sinim * a7 + cosim * a8;
        double a6 = -sinim * a9 + cosim * a10;
        
        double x1 = a1 * cosomm + a2 * sinomm;
        double x2 = a3 * cosomm + a4 * sinomm;
        double x3 = -a1 * sinomm + a2 * cosomm;
        double x4 = -a3 * sinomm + a4 * cosomm;
        double x5 = a5 * sinomm;
        double x6 = a6 * sinomm;
        double x7 = a5 * cosomm;
        double x8 = a6 * cosomm;
        
        z31 = 12.0 * x1 * x1 - 3.0 * x3 * x3;
        z32 = 24.0 * x1 * x2 - 6.0 * x3 * x4;
        z33 = 12.0 * x2 * x2 - 3.0 * x4 * x4;
        z1 = 3.0 * (a1 * a1 + a2 * a2) + z31 * emsq;
        z2 = 6.0 * (a1 * a3 + a2 * a4) + z32 * emsq;
        z3 = 3.0 * (a3 * a3 + a4 * a4) + z33 * emsq;
        z11 = -6.0 * a1 * a5 + emsq * (-24.0 * x1 * x7 - 6.0 * x3 * x5);
        z12 = -6.0 * (a1 * a6 + a3 * a5) + emsq * (-24.0 * (x2 * x7 + x1 * x8) - 6.0 * (x3 * x6 + x4 * x5));
        z13 = -6.0 * a3 * a6 + emsq * (-24.0 * x2 * x8 - 6.0 * x4 * x6);
        z21 = 6.0 * a2 * a5 + emsq * (24.0 * x1 * x5 - 6.0 * x3 * x7);
        z22 = 6.0 * (a4 * a5 + a2 * a6) + emsq * (24.0 * (x2 * x5 + x1 * x6) - 6.0 * (x4 * x7 + x3 * x8));
        z23 = 6.0 * a4 * a6 + emsq * (24.0 * x2 * x6 - 6.0 * x4 * x8);
        z1 = z1 + z1 + betasq * z31;
        z2 = z2 + z2 + betasq * z32;
        z3 = z3 + z3 + betasq * z33;
        s3 = cc * xnoi;
        s2 = -0.5 * s3 / rtemsq;
        s4 = s3 * rtemsq;
        s1 = -15.0 * em * s4;
        s5 = x1 * x3 + x2 * x4;
        s6 = x2 * x3 + x1 * x4;
        s7 = x2 * x4 - x1 * x3;
        
        // Handle solar vs lunar terms
        if (lsflg == 1) {
            ss1 = s1;
            ss2 = s2;
            ss3 = s3;
            ss4 = s4;
            ss5 = s5;
            ss6 = s6;
            ss7 = s7;
            sz1 = z1;
            sz2 = z2;
            sz3 = z3;
            sz11 = z11;
            sz12 = z12;
            sz13 = z13;
            sz21 = z21;
            sz22 = z22;
            sz23 = z23;
            sz31 = z31;
            sz32 = z32;
            sz33 = z33;
            
            // Calculate solar coefficients
            p.zmos = fmod(6.2565837 + 0.017201977 * day, TWOPI);
            
            // Switch to lunar values
            zcosg = zcosgl;
            zsing = zsingl;
            zcosi = zcosil;
            zsini = zsinil;
            zcosh = zcoshl * cnodm + zsinhl * snodm;
            zsinh = snodm * zcoshl - cnodm * zsinhl;
            cc = C1L;  // Use lunar coefficient from header
        }
    }
    
    // Calculate lunar term
    p.zmol = fmod(4.7199672 + 0.22997150 * day - gam, TWOPI);
    
    // Secular effects
    p.se2 = 2.0 * ss1 * ss6;
    p.se3 = 2.0 * ss1 * ss7;
    p.si2 = 2.0 * ss2 * sz12;
    p.si3 = 2.0 * ss2 * (sz13 - sz11);
    p.sl2 = -2.0 * ss3 * sz2;
    p.sl3 = -2.0 * ss3 * (sz3 - sz1);
    p.sl4 = -2.0 * ss3 * (-21.0 - 9.0 * emsq) * ZES;
    p.sgh2 = 2.0 * ss4 * sz32;
    p.sgh3 = 2.0 * ss4 * (sz33 - sz31);
    p.sgh4 = -18.0 * ss4 * ZES;
    p.sh2 = -2.0 * ss2 * sz22;
    p.sh3 = -2.0 * ss2 * (sz23 - sz21);
    
    // Lunar effects
    p.ee2 = 2.0 * s1 * s6;
    p.e3 = 2.0 * s1 * s7;
    p.xi2 = 2.0 * s2 * z12;
    p.xi3 = 2.0 * s2 * (z13 - z11);
    p.xl2 = -2.0 * s3 * z2;
    p.xl3 = -2.0 * s3 * (z3 - z1);
    p.xl4 = -2.0 * s3 * (-21.0 - 9.0 * emsq) * ZEL;
    p.xgh2 = 2.0 * s4 * z32;
    p.xgh3 = 2.0 * s4 * (z33 - z31);
    p.xgh4 = -18.0 * s4 * ZEL;
    p.xh2 = -2.0 * s2 * z22;
    p.xh3 = -2.0 * s2 * (z23 - z21);
    
    inclm = inclp;
    mm = argpp;
    argpm = argpp;
    nodem = nodep;
}

// ═══════════════════════════════════════════════════════════════════════════
// DSINIT - Deep Space Initialization
// Computes resonance and secular rates for deep space satellites
// ═══════════════════════════════════════════════════════════════════════════
__device__ void dsinit(
    double cosim, double emsq, double argpo, double s1, double s2,
    double s3, double s4, double s5, double sinim, double ss1,
    double ss2, double ss3, double ss4, double ss5, double sz1,
    double sz3, double sz11, double sz13, double sz21, double sz23,
    double sz31, double sz33, double t, double tc, double gsto,
    double mo, double mdot, double no, double nodeo, double nodedot,
    double xpidot, double z1, double z3, double z11, double z13,
    double z21, double z23, double z31, double z33, double ecco,
    double eccsq, double em, double argpm, double inclm, double mm,
    double nm, double nodem,
    Sgp4Params& p
) {
    double x2o3 = 2.0 / 3.0;
    
    // Initialize resonance flags
    p.irez = 0;
    if ((nm < 0.0052359877) && (nm > 0.0034906585)) {
        p.irez = 1;  // Synchronous (one-day) resonance
    }
    if ((nm >= 0.00826) && (nm <= 0.00924) && (em >= 0.5)) {
        p.irez = 2;  // Half-day resonance (12-hour orbit)
    }
    
    // Solar terms
    double ses = ss1 * ZNS * ss5;
    double sis = ss2 * ZNS * (sz11 + sz13);
    double sls = -ZNS * ss3 * (sz1 + sz3 - 14.0 - 6.0 * emsq);
    double sghs = ss4 * ZNS * (sz31 + sz33 - 6.0);
    double shs = -ZNS * ss2 * (sz21 + sz23);
    
    // Handle 180 degree inclination
    if ((inclm < INCLM_LIM) || (inclm > PI - INCLM_LIM)) {
        shs = 0.0;
    }
    if (sinim != 0.0) {
        shs = shs / sinim;
    }
    double sgs = sghs - cosim * shs;
    
    // Lunar terms
    p.dedt = ses + s1 * ZNL * s5;
    p.didt = sis + s2 * ZNL * (z11 + z13);
    p.dmdt = sls - ZNL * s3 * (z1 + z3 - 14.0 - 6.0 * emsq);
    double sghl = s4 * ZNL * (z31 + z33 - 6.0);
    double shll = -ZNL * s2 * (z21 + z23);
    
    if ((inclm < INCLM_LIM) || (inclm > PI - INCLM_LIM)) {
        shll = 0.0;
    }
    
    p.domdt = sgs + sghl;
    p.dnodt = shs;
    if (sinim != 0.0) {
        p.domdt = p.domdt - cosim / sinim * shll;
        p.dnodt = p.dnodt + shll / sinim;
    }
    
    // Calculate resonance effects
    double dndt = 0.0;
    double theta = fmod(gsto + tc * RPTIM, TWOPI);
    
    em = em + p.dedt * t;
    inclm = inclm + p.didt * t;
    argpm = argpm + p.domdt * t;
    nodem = nodem + p.dnodt * t;
    mm = mm + p.dmdt * t;
    
    // Initialize resonance terms
    if (p.irez != 0) {
        double aonv = pow(nm / XKE, x2o3);
        
        if (p.irez == 2) {
            // Half-day resonance (12-hour)
            double cosisq = cosim * cosim;
            double emo = em;
            em = ecco;
            double emsqo = emsq;
            emsq = eccsq;
            double eoc = em * emsq;
            double g201 = -0.306 - (em - 0.64) * 0.440;
            
            double g211, g310, g322, g410, g422, g520, g521, g532, g533;
            
            if (em <= 0.65) {
                g211 = 3.616 - 13.2470 * em + 16.2900 * emsq;
                g310 = -19.302 + 117.3900 * em - 228.4190 * emsq + 156.5910 * eoc;
                g322 = -18.9068 + 109.7927 * em - 214.6334 * emsq + 146.5816 * eoc;
                g410 = -41.122 + 242.6940 * em - 471.0940 * emsq + 313.9530 * eoc;
                g422 = -146.407 + 841.8800 * em - 1629.014 * emsq + 1083.4350 * eoc;
                g520 = -532.114 + 3017.977 * em - 5740.032 * emsq + 3708.2760 * eoc;
            } else {
                g211 = -72.099 + 331.819 * em - 508.738 * emsq + 266.724 * eoc;
                g310 = -346.844 + 1582.851 * em - 2415.925 * emsq + 1246.113 * eoc;
                g322 = -342.585 + 1554.908 * em - 2366.899 * emsq + 1215.972 * eoc;
                g410 = -1052.797 + 4758.686 * em - 7193.992 * emsq + 3651.957 * eoc;
                g422 = -3581.690 + 16178.110 * em - 24462.770 * emsq + 12422.520 * eoc;
                if (em > 0.715) {
                    g520 = -5149.66 + 29936.92 * em - 54087.36 * emsq + 31324.56 * eoc;
                } else {
                    g520 = 1464.74 - 4664.75 * em + 3763.64 * emsq;
                }
            }
            
            if (em < 0.7) {
                g533 = -919.2277 + 4988.61 * em - 9064.77 * emsq + 5542.21 * eoc;
                g521 = -822.71072 + 4568.6173 * em - 8491.4146 * emsq + 5337.524 * eoc;
                g532 = -853.666 + 4690.25 * em - 8624.77 * emsq + 5341.4 * eoc;
            } else {
                g533 = -37995.78 + 161616.52 * em - 229838.2 * emsq + 109377.94 * eoc;
                g521 = -51752.104 + 218913.95 * em - 309468.16 * emsq + 146349.42 * eoc;
                g532 = -40023.88 + 170470.89 * em - 242699.48 * emsq + 115605.82 * eoc;
            }
            
            double sini2 = sinim * sinim;
            double f220 = 0.75 * (1.0 + 2.0 * cosim + cosisq);
            double f221 = 1.5 * sini2;
            double f321 = 1.875 * sinim * (1.0 - 2.0 * cosim - 3.0 * cosisq);
            double f322 = -1.875 * sinim * (1.0 + 2.0 * cosim - 3.0 * cosisq);
            double f441 = 35.0 * sini2 * f220;
            double f442 = 39.3750 * sini2 * sini2;
            double f522 = 9.84375 * sinim * (sini2 * (1.0 - 2.0 * cosim - 5.0 * cosisq) +
                          0.33333333 * (-2.0 + 4.0 * cosim + 6.0 * cosisq));
            double f523 = sinim * (4.92187512 * sini2 * (-2.0 - 4.0 * cosim + 10.0 * cosisq) +
                          6.56250012 * (1.0 + 2.0 * cosim - 3.0 * cosisq));
            double f542 = 29.53125 * sinim * (2.0 - 8.0 * cosim + cosisq * (-12.0 + 8.0 * cosim + 10.0 * cosisq));
            double f543 = 29.53125 * sinim * (-2.0 - 8.0 * cosim + cosisq * (12.0 + 8.0 * cosim - 10.0 * cosisq));
            
            double xno2 = nm * nm;
            double ainv2 = aonv * aonv;
            double temp1 = 3.0 * xno2 * ainv2;
            double temp = temp1 * ROOT22;
            p.d2201 = temp * f220 * g201;
            p.d2211 = temp * f221 * g211;
            temp1 = temp1 * aonv;
            temp = temp1 * ROOT32;
            p.d3210 = temp * f321 * g310;
            p.d3222 = temp * f322 * g322;
            temp1 = temp1 * aonv;
            temp = 2.0 * temp1 * ROOT44;
            p.d4410 = temp * f441 * g410;
            p.d4422 = temp * f442 * g422;
            temp1 = temp1 * aonv;
            temp = temp1 * ROOT52;
            p.d5220 = temp * f522 * g520;
            p.d5232 = temp * f523 * g532;
            temp = 2.0 * temp1 * ROOT54;
            p.d5421 = temp * f542 * g521;
            p.d5433 = temp * f543 * g533;
            p.xlamo = fmod(mo + nodeo + nodeo - theta - theta, TWOPI);
            p.xfact = mdot + p.dmdt + 2.0 * (nodedot + p.dnodt - RPTIM) - no;
            em = emo;
            emsq = emsqo;
        }
        
        if (p.irez == 1) {
            // Synchronous (one-day) resonance
            double g200 = 1.0 + emsq * (-2.5 + 0.8125 * emsq);
            double g310 = 1.0 + 2.0 * emsq;
            double g300 = 1.0 + emsq * (-6.0 + 6.60937 * emsq);
            double f220 = 0.75 * (1.0 + cosim) * (1.0 + cosim);
            double f311 = 0.9375 * sinim * sinim * (1.0 + 3.0 * cosim) - 0.75 * (1.0 + cosim);
            double f330 = 1.0 + cosim;
            f330 = 1.875 * f330 * f330 * f330;
            p.del1 = 3.0 * nm * nm * aonv * aonv;
            p.del2 = 2.0 * p.del1 * f220 * g200 * Q22;
            p.del3 = 3.0 * p.del1 * f330 * g300 * Q33 * aonv;
            p.del1 = p.del1 * f311 * g310 * Q31 * aonv;
            p.xlamo = fmod(mo + nodeo + argpo - theta, TWOPI);
            p.xfact = mdot + xpidot - RPTIM + p.dmdt + p.domdt + p.dnodt - no;
        }
        
        // Initialize resonance variables for later
        p.xli = p.xlamo;
        p.xni = no;
        p.atime = 0.0;
        nm = no + dndt;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DPPER - Deep Space Periodic Contributions
// Adds lunar-solar periodic perturbations to orbital elements
// ═══════════════════════════════════════════════════════════════════════════
__device__ void dpper(
    double inclo, bool init, double t,
    double& ep, double& inclp, double& nodep, double& argpp, double& mp,
    Sgp4Params& p,  // Non-const: need to store baseline periodics when init=true
    bool debug = false  // Add debug parameter
) {
    // Calculate time-dependent variables
    double zm = p.zmos + ZNS * t;
    if (init) {
        zm = p.zmos;
    }
    double zf = zm + 2.0 * ZES * sin(zm);
    double sinzf = sin(zf);
    double f2 = 0.5 * sinzf * sinzf - 0.25;
    double coszf = cos(zf);
    double f3 = -0.5 * sinzf * coszf;

    double ses = p.se2 * f2 + p.se3 * f3;
    double sis = p.si2 * f2 + p.si3 * f3;
    double sls = p.sl2 * f2 + p.sl3 * f3 + p.sl4 * sinzf;
    double sghs = p.sgh2 * f2 + p.sgh3 * f3 + p.sgh4 * sinzf;
    double shs = p.sh2 * f2 + p.sh3 * f3;
    
    zm = p.zmol + ZNL * t;
    if (init) {
        zm = p.zmol;
    }
    zf = zm + 2.0 * ZEL * sin(zm);
    sinzf = sin(zf);
    f2 = 0.5 * sinzf * sinzf - 0.25;
    coszf = cos(zf);
    f3 = -0.5 * sinzf * coszf;

    double sel = p.ee2 * f2 + p.e3 * f3;
    double sil = p.xi2 * f2 + p.xi3 * f3;
    double sll = p.xl2 * f2 + p.xl3 * f3 + p.xl4 * sinzf;
    double sghl = p.xgh2 * f2 + p.xgh3 * f3 + p.xgh4 * sinzf;
    double shll = p.xh2 * f2 + p.xh3 * f3;
    
    double pe = ses + sel;
    double pinc = sis + sil;
    double pl = sls + sll;
    double pgh = sghs + sghl;
    double ph = shs + shll;
    
    if (debug && !init) {
        printf("\n--- DPPER Debug (t=%.2f) ---\n", t);
        printf("Solar: zm=%.10f, zf=%.10f\n", p.zmos + ZNS * t, p.zmos + ZNS * t + 2.0 * ZES * sin(p.zmos + ZNS * t));
        printf("  sis = si2*f2 + si3*f3 = %.15e\n", sis);
        printf("  si2=%.15e, si3=%.15e\n", p.si2, p.si3);
        printf("Lunar: zm=%.10f, zf=%.10f\n", p.zmol + ZNL * t, p.zmol + ZNL * t + 2.0 * ZEL * sin(p.zmol + ZNL * t));
        printf("  sil = xi2*f2 + xi3*f3 = %.15e\n", sil);
        printf("  xi2=%.15e, xi3=%.15e\n", p.xi2, p.xi3);
        printf("pinc = sis + sil = %.15e rad (%.10f deg)\n", pinc, pinc * RAD2DEG);
        printf("Baseline: peo=%.15e, pinco=%.15e\n", p.peo, p.pinco);
    }
    
    if (init) {
        // Store baseline periodic values at epoch for later subtraction
        p.peo = pe;
        p.pinco = pinc;
        p.plo = pl;
        p.pgho = pgh;
        p.pho = ph;
    } else {
        pe = pe - p.peo;
        pinc = pinc - p.pinco;
        pl = pl - p.plo;
        pgh = pgh - p.pgho;
        ph = ph - p.pho;
        inclp = inclp + pinc;
        ep = ep + pe;
        double sinip = sin(inclp);
        double cosip = cos(inclp);
        
        // Apply periodic variations using improved method for numerical stability
        // Account for nearly polar/equatorial orbits
        if (inclp >= 0.2) {
            ph = ph / sinip;
            pgh = pgh - cosip * ph;
            argpp = argpp + pgh;
            nodep = nodep + ph;
            mp = mp + pl;
        } else {
            // Use lyddane modification for low inclination
            double sinop = sin(nodep);
            double cosop = cos(nodep);
            double alfdp = sinip * sinop;
            double betdp = sinip * cosop;
            double dalf = ph * cosop + pinc * cosip * sinop;
            double dbet = -ph * sinop + pinc * cosip * cosop;
            alfdp = alfdp + dalf;
            betdp = betdp + dbet;
            // Use python-sgp4 style signed modulo: preserve sign for negative values
            // nodep % twopi if nodep >= 0.0 else -(-nodep % twopi)
            if (nodep >= 0.0) {
                nodep = fmod(nodep, TWOPI);
            } else {
                nodep = -fmod(-nodep, TWOPI);
            }
            // Match python-sgp4 formula exactly: xls = mp + argpp + pl + pgh + (cosip - pinc * sinip) * nodep
            double xls = mp + argpp + pl + pgh + (cosip - pinc * sinip) * nodep;
            double xnoh = nodep;
            nodep = atan2(alfdp, betdp);
            if (fabs(xnoh - nodep) > PI) {
                if (nodep < xnoh) {
                    nodep = nodep + TWOPI;
                } else {
                    nodep = nodep - TWOPI;
                }
            }
            mp = mp + pl;
            argpp = xls - mp - cosip * nodep;
        }
    }

    // Debug output: Print final dpper output values
    if (debug && !init) {
        printf("\n=== DPPER OUTPUT (GPU) ===\n");
        printf("ep (em):      %.16f\n", ep);
        printf("inclp (inclm): %.16f rad\n", inclp);
        printf("nodep (nodem): %.16f rad\n", nodep);
        printf("argpp (argpm): %.16f rad\n", argpp);
        printf("mp (mm):      %.16f rad\n", mp);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DSPACE - Deep Space Secular Effects
// Computes secular perturbations from resonance effects
// ═══════════════════════════════════════════════════════════════════════════
__device__ void dspace(
    double t, double tc,
    double& em, double& argpm, double& inclm, double& mm, double& nm, double& nodem,
    Sgp4Params& p
) {
    // Resonance phase constants (fixed values from Spacetrack Report #3)
    const double fasx2 = 0.13130908;
    const double fasx4 = 2.8843198;
    const double fasx6 = 0.37448087;
    
    // Use header constants for G-values and step sizes
    // G22, G32, G44, G52, G54, STEP, STEPN, STEP2 from sgp4_constants.cuh
    
    double delt, xni, xli, atime;
    double ft = 0.0;
    double xndt = 0.0;
    double xnddt = 0.0;
    double xldot = 0.0;
    double theta = fmod(p.gsto + tc * RPTIM, TWOPI);
    
    em = em + p.dedt * t;
    inclm = inclm + p.didt * t;
    argpm = argpm + p.domdt * t;
    nodem = nodem + p.dnodt * t;
    mm = mm + p.dmdt * t;
    
    // Calculate resonance effects
    if (p.irez != 0) {
        if ((p.atime == 0.0) || (t * p.atime <= 0.0) || (fabs(t) < fabs(p.atime))) {
            p.atime = 0.0;
            p.xni = p.no_unkozai;
            p.xli = p.xlamo;
        }
        
        // Determine step direction
        if (t > 0.0) {
            delt = STEP;
        } else {
            delt = STEPN;
        }
        
        atime = p.atime;
        xni = p.xni;
        xli = p.xli;
        
        // Integrate resonance equations
        while (true) {
            if (p.irez != 2) {
                // Synchronous resonance
                xndt = p.del1 * sin(xli - fasx2) +
                       p.del2 * sin(2.0 * (xli - fasx4)) +
                       p.del3 * sin(3.0 * (xli - fasx6));
                xldot = xni + p.xfact;
                xnddt = p.del1 * cos(xli - fasx2) +
                        2.0 * p.del2 * cos(2.0 * (xli - fasx4)) +
                        3.0 * p.del3 * cos(3.0 * (xli - fasx6));
                xnddt = xnddt * xldot;
                
                if (fabs(t - atime) < STEP) {
                    ft = t - atime;
                    break;
                }
                xli = xli + xldot * delt + xndt * STEP2;
                xni = xni + xndt * delt + xnddt * STEP2;
                atime = atime + delt;
            } else {
                // Half-day resonance
                double xomi = p.argpo + p.argpdot * atime;
                double x2omi = xomi + xomi;
                double x2li = xli + xli;
                xndt = p.d2201 * sin(x2omi + xli - G22) +
                       p.d2211 * sin(xli - G22) +
                       p.d3210 * sin(xomi + xli - G32) +
                       p.d3222 * sin(-xomi + xli - G32) +
                       p.d4410 * sin(x2omi + x2li - G44) +
                       p.d4422 * sin(x2li - G44) +
                       p.d5220 * sin(xomi + xli - G52) +
                       p.d5232 * sin(-xomi + xli - G52) +
                       p.d5421 * sin(xomi + x2li - G54) +
                       p.d5433 * sin(-xomi + x2li - G54);
                xldot = xni + p.xfact;
                xnddt = p.d2201 * cos(x2omi + xli - G22) +
                        p.d2211 * cos(xli - G22) +
                        p.d3210 * cos(xomi + xli - G32) +
                        p.d3222 * cos(-xomi + xli - G32) +
                        p.d5220 * cos(xomi + xli - G52) +
                        p.d5232 * cos(-xomi + xli - G52) +
                        2.0 * (p.d4410 * cos(x2omi + x2li - G44) +
                               p.d4422 * cos(x2li - G44) +
                               p.d5421 * cos(xomi + x2li - G54) +
                               p.d5433 * cos(-xomi + x2li - G54));
                xnddt = xnddt * xldot;
                
                if (fabs(t - atime) < STEP) {
                    ft = t - atime;
                    break;
                }
                xli = xli + xldot * delt + xndt * STEP2;
                xni = xni + xndt * delt + xnddt * STEP2;
                atime = atime + delt;
            }
        }
        
        // Update state
        p.atime = atime;
        p.xni = xni;
        p.xli = xli;
        
        // Apply Taylor series expansion for mean motion and mean longitude
        nm = xni + xndt * ft + xnddt * ft * ft * 0.5;
        double xl = xli + xldot * ft + xndt * ft * ft * 0.5;
        
        if (p.irez != 1) {
            mm = xl - 2.0 * nodem + 2.0 * theta;
        } else {
            mm = xl - nodem - argpm + theta;
        }
        mm = fmod(mm, TWOPI);
    }
}

#endif // SGP4_DEEPSPACE_CUH
