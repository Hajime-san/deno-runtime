const real = [0.0, 0.5];
const imag = [0.00467, -0.872];
const fft = MyCoolMath.fft(real, imag, 2);
const ifft = MyCoolMath.ifft(fft[0], fft[1], 2);

console.log(ifft);
