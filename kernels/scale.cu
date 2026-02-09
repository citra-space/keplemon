// Scale kernel for in-VRAM unit conversion of position/velocity buffers

extern "C" __global__ void scale_f64_kernel(double* data, int n, double factor) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) {
        data[idx] *= factor;
    }
}
