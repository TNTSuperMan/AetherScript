import { file, serve } from "bun";
import index from "./index.html";
serve({
    routes: {
        "/*": index,
        "/data1": file(`${import.meta.dir}/../data/obj.json`),
        "/data2": file(`${import.meta.dir}/../data/tuple.json`),
    }
})