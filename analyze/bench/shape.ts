import { bench } from "./base";

const tuple1: [Map<number, string>, number] = [new Map(), 0];
const tuple2: [string, Set<number>] = [crypto.randomUUID(), new Set()];
const obj1 = {
    map: new Map<number, string>(),
    id: 0,
};
const obj2 = {
    uuid: crypto.randomUUID(),
    set: new Set<number>(),
};

bench("tuple", () => {}, () => {
    let id = tuple1[1]++;
    if (tuple2[1].has(id)) {
        id += 1;
    } else {
        tuple2[1].add(id);
    }
    tuple1[0].set(id, tuple2[0]);
    tuple2[0] = crypto.randomUUID();
});

bench("obj", () => {}, () => {
    let id = obj1.id++;
    if (obj2.set.has(id)) {
        id += 1;
    } else {
        obj2.set.add(id);
    }
    obj1.map.set(id, obj2.uuid);
    obj2.uuid = crypto.randomUUID();
});
