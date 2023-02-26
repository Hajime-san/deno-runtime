((globalThis) => {
  const core = Deno.core;

  core.initializeAsyncOps();

  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }

  globalThis.console = {
    log: (...args) => {
      core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
      core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
  };

  globalThis.MyCoolMath = {
    fft: (real, imag, sample) => {
      return core.ops.op_my_cool_math_fft(real, imag, sample);
    },
    ifft: (real, imag, sample) => {
      return core.ops.op_my_cool_math_ifft(real, imag, sample);
    },
  };
})(globalThis);
