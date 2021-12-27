import init, { mine, verify } from '../../pkg/web/sha256_pow.js';

await init();

const target = "00000021e800000000000000000000000000000000000000000000000000000";

const powBuffer = new TextEncoder().encode("Hello, PoW!");

window.addEventListener("pow-event", (hex) => {
    console.log(hex)
});

const pow = await mine(powBuffer, target);

console.log("Can verify PoW?", verify(pow, target));

