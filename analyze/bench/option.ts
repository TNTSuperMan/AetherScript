import { bench } from "./base";

const obj = () => ({
    id: crypto.randomUUID(),
    len: 36,
});

bench("union", () => Math.random() > 0.5 ? obj() : null, d => {
    if (d === null) {
        return `None\n`;
    } else {
        return `Some(${d.id}, len: ${d.len})\n`;
    }
});

bench("option", () => Math.random() > 0.5 ? {
        is_some: true,
        value: obj(),
    } : {
        is_some: false,
        value: null,
    }, d => {
    if (d.is_some) {
        return `Some(${d.value!.id}, len: ${d.value!.len})\n`;
    } else {
        return `None\n`;
    }
});

const map = new WeakMap<WeakKey, ReturnType<typeof obj>>();

bench("map", () => {
    if (Math.random() > 0.5) {
        const k = {
            is_some: true,
        };
        map.set(k, obj());
        return k;
    } else {
        return {
            is_some: false,
        }
    }
}, d => {
    if (d.is_some) {
        const z = map.get(d)!;
        return `Some(${z.id}, len: ${z.len})\n`;
    } else {
        return `None\n`;
    }
});
