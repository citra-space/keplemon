// ═══════════════════════════════════════════════════════════════════════════════════
// SDP4 Interpolated Propagator - Device Functions
// ═══════════════════════════════════════════════════════════════════════════════════
//
// This file implements the SDP4-compatible interpolated propagator for GPU.
// Key innovation: Pre-sampled resonance interpolation eliminates iterative loops.
//
// Algorithm:
// 1. Initialization: DSCOM → DSINIT → Pre-sample resonance at regular intervals
// 2. Propagation: Apply secular effects → Interpolate resonance → DPPER → Convert to Cartesian
//
// References:
// - Hoots & Roehrich, "Spacetrack Report No. 3" (1980)
// - Vallado et al., "Revisiting Spacetrack Report #3" AIAA 2006-6753
//
// ═══════════════════════════════════════════════════════════════════════════════════

#ifndef SDP4_INTERPOLATED_CUH
#define SDP4_INTERPOLATED_CUH

#include "tle_propagator_constants.cuh"
#include "tle_propagator_types.cuh"
#include "sdp4_interpolated_types.cuh"

// ═══════════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ __forceinline__ double sdp4_fmod_2pi(double x) {
    double result = fmod(x, TWOPI);
    if (result < 0.0) result += TWOPI;
    return result;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DSCOM - Deep Space Common Items (adapted for Sdp4InterpolatedParams)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_dscom(
    double epoch, double ep, double argpp, double tc, double inclp,
    double nodep, double np,
    Sdp4InterpolatedParams& p,
    // Intermediate outputs needed by DSINIT
    double& sinim, double& cosim, double& emsq, double& rtemsq,
    double& s1, double& s2, double& s3, double& s4, double& s5, double& s6, double& s7,
    double& ss1, double& ss2, double& ss3, double& ss4, double& ss5, double& ss6, double& ss7,
    double& sz1, double& sz2, double& sz3, double& sz11, double& sz12, double& sz13,
    double& sz21, double& sz22, double& sz23, double& sz31, double& sz32, double& sz33,
    double& z1, double& z2, double& z3, double& z11, double& z12, double& z13,
    double& z21, double& z22, double& z23, double& z31, double& z32, double& z33
) {
    // Compute sine/cosine of orbital elements
    double snodm = sin(nodep);
    double cnodm = cos(nodep);
    double sinomm = sin(argpp);
    double cosomm = cos(argpp);
    sinim = sin(inclp);
    cosim = cos(inclp);
    emsq = ep * ep;
    double betasq = 1.0 - emsq;
    rtemsq = sqrt(betasq);

    // Initialize periodic terms
    p.peo = 0.0;
    p.pinco = 0.0;
    p.plo = 0.0;
    p.pgho = 0.0;
    p.pho = 0.0;

    // Day calculation (fraction of day since Jan 0, 1950)
    double day = epoch + 18261.5 + tc / 1440.0;

    // Lunar parameters
    double xnodce = fmod(4.5236020 - 9.2422029e-4 * day, TWOPI);
    double stem = sin(xnodce);
    double ctem = cos(xnodce);
    double zcosil = 0.91375164 - 0.03568096 * ctem;
    double zsinil = sqrt(1.0 - zcosil * zcosil);
    double zsinhl = 0.089683511 * stem / zsinil;
    double zcoshl = sqrt(1.0 - zsinhl * zsinhl);
    double gam = 5.8351514 + 0.0019443680 * day;
    double zx = ZSINIS * stem / zsinil;
    double zy = zcoshl * ctem + ZCOSIS * zsinhl * stem;
    zx = atan2(zx, zy);
    zx = gam + zx - xnodce;
    double zcosgl = cos(zx);
    double zsingl = sin(zx);

    // Solar terms
    double zcosg = ZCOSGS;
    double zsing = ZSINGS;
    double zcosi = ZCOSIS;
    double zsini = ZSINIS;
    double zcosh = cnodm;
    double zsinh = snodm;
    double cc = C1SS;
    double xnoi = 1.0 / np;

    // Loop for solar (lsflg=1) and lunar (lsflg=2) terms
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
        s1 = -15.0 * ep * s4;
        s5 = x1 * x3 + x2 * x4;
        s6 = x2 * x3 + x1 * x4;
        s7 = x2 * x4 - x1 * x3;

        if (lsflg == 1) {
            ss1 = s1; ss2 = s2; ss3 = s3; ss4 = s4; ss5 = s5; ss6 = s6; ss7 = s7;
            sz1 = z1; sz2 = z2; sz3 = z3;
            sz11 = z11; sz12 = z12; sz13 = z13;
            sz21 = z21; sz22 = z22; sz23 = z23;
            sz31 = z31; sz32 = z32; sz33 = z33;

            // Solar mean anomaly at epoch
            p.zmos = fmod(6.2565837 + 0.017201977 * day, TWOPI);

            // Switch to lunar values
            zcosg = zcosgl;
            zsing = zsingl;
            zcosi = zcosil;
            zsini = zsinil;
            zcosh = zcoshl * cnodm + zsinhl * snodm;
            zsinh = snodm * zcoshl - cnodm * zsinhl;
            cc = C1L;
        }
    }

    // Lunar mean anomaly at epoch
    p.zmol = fmod(4.7199672 + 0.22997150 * day - gam, TWOPI);

    // Store secular coefficients
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
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DSINIT - Deep Space Initialization (adapted for Sdp4InterpolatedParams)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_dsinit(
    double cosim, double emsq, double argpo, double s1, double s2,
    double s3, double s4, double s5, double sinim, double ss1,
    double ss2, double ss3, double ss4, double ss5, double sz1,
    double sz3, double sz11, double sz13, double sz21, double sz23,
    double sz31, double sz33, double gsto, double mo, double mdot,
    double no, double nodeo, double nodedot, double xpidot,
    double z1, double z3, double z11, double z13, double z21, double z23,
    double z31, double z33, double ecco, double eccsq, double em,
    double inclm, double nm,
    Sdp4InterpolatedParams& p
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

    // Calculate resonance terms
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
            p.xlamo = fmod(mo + nodeo + nodeo - gsto - gsto, TWOPI);
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
            p.xlamo = fmod(mo + nodeo + argpo - gsto, TWOPI);
            p.xfact = mdot + xpidot - RPTIM + p.dmdt + p.domdt + p.dnodt - no;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DPPER - Deep Space Periodic Contributions (adapted for Sdp4InterpolatedParams)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_dpper(
    double inclo, bool init, double t,
    double& ep, double& inclp, double& nodep, double& argpp, double& mp,
    Sdp4InterpolatedParams& p
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

    if (init) {
        // Store baseline periodic values at epoch
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

        // Apply periodic variations
        if (inclp >= 0.2) {
            ph = ph / sinip;
            pgh = pgh - cosip * ph;
            argpp = argpp + pgh;
            nodep = nodep + ph;
            mp = mp + pl;
        } else {
            // Lyddane modification for low inclination
            double sinop = sin(nodep);
            double cosop = cos(nodep);
            double alfdp = sinip * sinop;
            double betdp = sinip * cosop;
            double dalf = ph * cosop + pinc * cosip * sinop;
            double dbet = -ph * sinop + pinc * cosip * cosop;
            alfdp = alfdp + dalf;
            betdp = betdp + dbet;
            if (nodep >= 0.0) {
                nodep = fmod(nodep, TWOPI);
            } else {
                nodep = -fmod(-nodep, TWOPI);
            }
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
}

// ═══════════════════════════════════════════════════════════════════════════════════
// RESONANCE INTEGRATION STEP
// Computes one step of the iterative resonance integration
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_resonance_step(
    int irez, double argpo, double argpdot,
    double del1, double del2, double del3,
    double d2201, double d2211, double d3210, double d3222,
    double d4410, double d4422, double d5220, double d5232,
    double d5421, double d5433, double xfact,
    double& xli, double& xni, double delt,
    double& xndt, double& xldot, double& xnddt
) {
    // Resonance phase constants
    const double fasx2 = 0.13130908;
    const double fasx4 = 2.8843198;
    const double fasx6 = 0.37448087;

    if (irez == 1) {
        // Synchronous resonance
        xndt = del1 * sin(xli - fasx2) +
               del2 * sin(2.0 * (xli - fasx4)) +
               del3 * sin(3.0 * (xli - fasx6));
        xldot = xni + xfact;
        xnddt = del1 * cos(xli - fasx2) +
                2.0 * del2 * cos(2.0 * (xli - fasx4)) +
                3.0 * del3 * cos(3.0 * (xli - fasx6));
        xnddt = xnddt * xldot;
    } else {
        // Half-day resonance
        double xomi = argpo + argpdot * 0.0;  // atime=0 for initial sample
        double x2omi = xomi + xomi;
        double x2li = xli + xli;
        xndt = d2201 * sin(x2omi + xli - G22) +
               d2211 * sin(xli - G22) +
               d3210 * sin(xomi + xli - G32) +
               d3222 * sin(-xomi + xli - G32) +
               d4410 * sin(x2omi + x2li - G44) +
               d4422 * sin(x2li - G44) +
               d5220 * sin(xomi + xli - G52) +
               d5232 * sin(-xomi + xli - G52) +
               d5421 * sin(xomi + x2li - G54) +
               d5433 * sin(-xomi + x2li - G54);
        xldot = xni + xfact;
        xnddt = d2201 * cos(x2omi + xli - G22) +
                d2211 * cos(xli - G22) +
                d3210 * cos(xomi + xli - G32) +
                d3222 * cos(-xomi + xli - G32) +
                d5220 * cos(xomi + xli - G52) +
                d5232 * cos(-xomi + xli - G52) +
                2.0 * (d4410 * cos(x2omi + x2li - G44) +
                       d4422 * cos(x2li - G44) +
                       d5421 * cos(xomi + x2li - G54) +
                       d5433 * cos(-xomi + x2li - G54));
        xnddt = xnddt * xldot;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PRE-SAMPLE RESONANCE
// Computes resonance at regular intervals for later interpolation
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_presample_resonance(
    const Sdp4InterpolatedParams& p,
    Sdp4ResonanceSample* samples,
    int& n_samples
) {
    if (p.irez == 0) {
        // No resonance - no samples needed
        n_samples = 0;
        return;
    }

    // Sample from -30 days to +30 days at 6-hour intervals
    double t_min = -SDP4_MAX_PROPAGATION_SPAN_MIN;
    double t_max = SDP4_MAX_PROPAGATION_SPAN_MIN;
    double dt = SDP4_RESONANCE_SAMPLE_INTERVAL_MIN;

    n_samples = 0;
    double xli = p.xlamo;
    double xni = p.no_unkozai;

    // Forward integration from epoch
    double atime = 0.0;
    int fwd_start = SDP4_MAX_RESONANCE_SAMPLES / 2;
    samples[fwd_start].xli = xli;
    samples[fwd_start].xni = xni;
    samples[fwd_start].atime = 0.0;

    // Forward samples (positive time)
    for (int i = 1; i <= SDP4_MAX_RESONANCE_SAMPLES / 2; i++) {
        double xndt, xldot, xnddt;
        sdp4_resonance_step(p.irez, p.argpo, p.argpdot,
                            p.del1, p.del2, p.del3,
                            p.d2201, p.d2211, p.d3210, p.d3222,
                            p.d4410, p.d4422, p.d5220, p.d5232,
                            p.d5421, p.d5433, p.xfact,
                            xli, xni, dt, xndt, xldot, xnddt);

        // Integrate
        xli = xli + xldot * dt + xndt * STEP2;
        xni = xni + xndt * dt + xnddt * STEP2;
        atime = atime + dt;

        int idx = fwd_start + i;
        if (idx < SDP4_MAX_RESONANCE_SAMPLES) {
            samples[idx].xli = xli;
            samples[idx].xni = xni;
            samples[idx].atime = atime;
        }
    }

    // Reset for backward integration
    xli = p.xlamo;
    xni = p.no_unkozai;
    atime = 0.0;

    // Backward samples (negative time)
    for (int i = 1; i <= SDP4_MAX_RESONANCE_SAMPLES / 2; i++) {
        double xndt, xldot, xnddt;
        sdp4_resonance_step(p.irez, p.argpo, p.argpdot,
                            p.del1, p.del2, p.del3,
                            p.d2201, p.d2211, p.d3210, p.d3222,
                            p.d4410, p.d4422, p.d5220, p.d5232,
                            p.d5421, p.d5433, p.xfact,
                            xli, xni, -dt, xndt, xldot, xnddt);

        // Integrate backwards
        xli = xli + xldot * (-dt) + xndt * STEP2;
        xni = xni + xndt * (-dt) + xnddt * STEP2;
        atime = atime - dt;

        int idx = fwd_start - i;
        if (idx >= 0) {
            samples[idx].xli = xli;
            samples[idx].xni = xni;
            samples[idx].atime = atime;
        }
    }

    n_samples = SDP4_MAX_RESONANCE_SAMPLES;
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INTERPOLATE RESONANCE
// Uses linear interpolation on pre-sampled resonance values
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_interpolate_resonance(
    double t,
    const Sdp4ResonanceSample* samples,
    int n_samples,
    double sample_t0,
    double sample_dt,
    double& xli_out, double& xni_out
) {
    if (n_samples == 0) {
        xli_out = 0.0;
        xni_out = 0.0;
        return;
    }

    // Find the sample index
    double fidx = (t - sample_t0) / sample_dt;
    int idx0 = (int)fidx;
    if (idx0 < 0) idx0 = 0;
    if (idx0 >= n_samples - 1) idx0 = n_samples - 2;

    int idx1 = idx0 + 1;
    double frac = fidx - (double)idx0;
    if (frac < 0.0) frac = 0.0;
    if (frac > 1.0) frac = 1.0;

    // Linear interpolation
    xli_out = samples[idx0].xli + frac * (samples[idx1].xli - samples[idx0].xli);
    xni_out = samples[idx0].xni + frac * (samples[idx1].xni - samples[idx0].xni);
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DSPACE WITH INTERPOLATION
// Applies secular and interpolated resonance effects
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_dspace_interpolated(
    double t, double tc, double gsto,
    const Sdp4ResonanceSample* samples, int n_samples,
    double sample_t0, double sample_dt,
    double& em, double& argpm, double& inclm, double& mm, double& nm, double& nodem,
    const Sdp4InterpolatedParams& p
) {
    // Apply secular effects
    em = em + p.dedt * t;
    inclm = inclm + p.didt * t;
    argpm = argpm + p.domdt * t;
    nodem = nodem + p.dnodt * t;
    mm = mm + p.dmdt * t;

    // Apply resonance effects via interpolation
    if (p.irez != 0 && n_samples > 0) {
        double theta = fmod(gsto + tc * RPTIM, TWOPI);

        double xli, xni;
        sdp4_interpolate_resonance(t, samples, n_samples, sample_t0, sample_dt, xli, xni);

        // Apply resonance correction to mean motion
        nm = xni;

        // Compute mean anomaly from resonance
        if (p.irez == 1) {
            // Synchronous
            mm = xli - nodem - argpm + theta;
        } else {
            // Half-day
            mm = xli - 2.0 * nodem + 2.0 * theta;
        }
        mm = fmod(mm, TWOPI);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PROPAGATE SINGLE - Main propagation function (matches standard SDP4)
// ═══════════════════════════════════════════════════════════════════════════════════

__device__ void sdp4_propagate_single(
    const Sdp4InterpolatedParams& p,
    const Sdp4ResonanceSample* samples,
    double tsince,  // minutes since epoch
    double* x, double* y, double* z,
    double* vx, double* vy, double* vz,
    int* error_code
) {
    *error_code = 0;

    // Handle zero time case
    if (fabs(tsince) < 1e-12) {
        tsince = 0.0;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // UPDATE FOR SECULAR GRAVITY AND ATMOSPHERIC DRAG (match standard SGP4)
    // ═══════════════════════════════════════════════════════════════════════════

    double xmdf = p.mo + p.mdot * tsince;
    double argpdf = p.argpo + p.argpdot * tsince;
    double nodedf = p.nodeo + p.nodedot * tsince;

    double t2 = tsince * tsince;
    double tempa = 1.0 - p.cc1 * tsince;
    double tempe = p.bstar * p.cc4 * tsince;
    double templ = p.t2cof * t2;

    double nm = p.no_unkozai;
    double em = p.ecco;
    double inclm = p.inclo;

    // Deep space: use DSPACE (with interpolated resonance)
    double mm = xmdf;
    double argpm = argpdf;
    double nodem = nodedf;
    double tc = tsince;

    // Apply deep space secular effects via interpolation
    sdp4_dspace_interpolated(
        tsince, tc, p.gsto,
        samples, p.n_samples, p.sample_t0, p.sample_dt,
        em, argpm, inclm, mm, nm, nodem, p
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // APPLY DRAG AND SECULAR CORRECTIONS
    // ═══════════════════════════════════════════════════════════════════════════

    double am = pow(XKE / nm, X2O3) * tempa * tempa;
    nm = XKE / pow(am, 1.5);
    em = em - tempe;

    // Error checks
    if (em < 1.0e-6) em = 1.0e-6;
    if (em > 0.9999 || am < 0.95) {
        *error_code = 1;  // Satellite has decayed
        return;
    }

    // Apply templ to mean anomaly
    mm = mm + p.no_unkozai * templ;

    // ═══════════════════════════════════════════════════════════════════════════
    // DEEP SPACE PERIODIC EFFECTS (DPPER)
    // ═══════════════════════════════════════════════════════════════════════════

    sdp4_dpper(p.inclo, false, tsince, em, inclm, nodem, argpm, mm,
               const_cast<Sdp4InterpolatedParams&>(p));

    double xnode = nodem;

    // Normalize angles (following python-sgp4)
    double xlm = mm + argpm + xnode;
    xnode = fmod(xnode, TWOPI);
    argpm = fmod(argpm, TWOPI);
    xlm = fmod(xlm, TWOPI);
    mm = fmod(xlm - argpm - xnode, TWOPI);
    if (xnode < 0.0) xnode += TWOPI;
    if (mm < 0.0) mm += TWOPI;

    // ═══════════════════════════════════════════════════════════════════════════
    // LONG PERIOD PERIODICS
    // ═══════════════════════════════════════════════════════════════════════════

    double sinip = sin(inclm);
    double cosip = cos(inclm);

    // Recalculate aycof and xlcof using dpper-modified inclination
    double aycof_eff = -0.5 * J3OJ2 * sinip;
    double xlcof_eff;
    if (fabs(cosip + 1.0) > 1.5e-12) {
        xlcof_eff = -0.25 * J3OJ2 * sinip * (3.0 + 5.0 * cosip) / (1.0 + cosip);
    } else {
        xlcof_eff = -0.25 * J3OJ2 * sinip * (3.0 + 5.0 * cosip) / 1.5e-12;
    }

    double axnl = em * cos(argpm);
    double temp = 1.0 / (am * (1.0 - em * em));
    double aynl = em * sin(argpm) + temp * aycof_eff;
    double xl = mm + argpm + xnode + temp * xlcof_eff * axnl;

    // ═══════════════════════════════════════════════════════════════════════════
    // SOLVE KEPLER'S EQUATION
    // ═══════════════════════════════════════════════════════════════════════════

    double u = fmod(xl - xnode, TWOPI);
    double eo1 = u;
    double tem5 = 1.0;
    int ktr = 1;

    while (fabs(tem5) >= 1.0e-12 && ktr <= 10) {
        double sineo1 = sin(eo1);
        double coseo1 = cos(eo1);
        tem5 = 1.0 - coseo1 * axnl - sineo1 * aynl;
        tem5 = (u - aynl * coseo1 + axnl * sineo1 - eo1) / tem5;
        if (fabs(tem5) >= 0.95) {
            tem5 = tem5 > 0.0 ? 0.95 : -0.95;
        }
        eo1 = eo1 + tem5;
        ktr++;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SHORT PERIOD PERIODICS
    // ═══════════════════════════════════════════════════════════════════════════

    double sineo1 = sin(eo1);
    double coseo1 = cos(eo1);

    double ecose = axnl * coseo1 + aynl * sineo1;
    double esine = axnl * sineo1 - aynl * coseo1;
    double el2 = axnl * axnl + aynl * aynl;
    double pl = am * (1.0 - el2);

    if (pl < 0.0) {
        *error_code = 2;
        return;
    }

    double rl = am * (1.0 - ecose);
    double rdotl = sqrt(am) * esine / rl;
    double rvdotl = sqrt(pl) / rl;
    double betal = sqrt(1.0 - el2);
    temp = esine / (1.0 + betal);

    double sinu = am / rl * (sineo1 - aynl - axnl * temp);
    double cosu = am / rl * (coseo1 - axnl + aynl * temp);
    double su = atan2(sinu, cosu);

    double sin2u = (cosu + cosu) * sinu;
    double cos2u = 1.0 - 2.0 * sinu * sinu;

    temp = 1.0 / pl;
    double temp1 = 0.5 * J2 * temp;
    double temp2 = temp1 * temp;

    // Recalculate con41, x1mth2, x7thm1 using dpper-modified inclination
    double cosisq = cosip * cosip;
    double con41_eff = 3.0 * cosisq - 1.0;
    double x1mth2_eff = 1.0 - cosisq;
    double x7thm1_eff = 7.0 * cosisq - 1.0;

    double mrt = rl * (1.0 - 1.5 * temp2 * betal * con41_eff) +
                 0.5 * temp1 * x1mth2_eff * cos2u;
    su = su - 0.25 * temp2 * x7thm1_eff * sin2u;
    double xnode_new = xnode + 1.5 * temp2 * cosip * sin2u;
    double xinc = inclm + 1.5 * temp2 * cosip * sinip * cos2u;
    double mvt = rdotl - nm * temp1 * x1mth2_eff * sin2u / XKE;
    double rvdot = rvdotl + nm * temp1 * (x1mth2_eff * cos2u + 1.5 * con41_eff) / XKE;

    // ═══════════════════════════════════════════════════════════════════════════
    // ORIENTATION VECTORS
    // ═══════════════════════════════════════════════════════════════════════════

    double sinsu = sin(su);
    double cossu = cos(su);
    double snod = sin(xnode_new);
    double cnod = cos(xnode_new);
    double sini = sin(xinc);
    double cosi = cos(xinc);

    double xmx = -snod * cosi;
    double xmy = cnod * cosi;
    double ux = xmx * sinsu + cnod * cossu;
    double uy = xmy * sinsu + snod * cossu;
    double uz = sini * sinsu;
    double vx_temp = xmx * cossu - cnod * sinsu;
    double vy_temp = xmy * cossu - snod * sinsu;
    double vz_temp = sini * cossu;

    // Position (km)
    *x = mrt * ux * RE;
    *y = mrt * uy * RE;
    *z = mrt * uz * RE;

    // Velocity (km/s)
    *vx = (mvt * ux + rvdot * vx_temp) * VKMPERSEC;
    *vy = (mvt * uy + rvdot * vy_temp) * VKMPERSEC;
    *vz = (mvt * uz + rvdot * vz_temp) * VKMPERSEC;

    // Validate output
    double r = sqrt((*x)*(*x) + (*y)*(*y) + (*z)*(*z));
    if (r < 100.0 || r > 1000000.0) {
        *error_code = 2;
    }
}

#endif // SDP4_INTERPOLATED_CUH
