export class Grid {
    private readonly contaienr:HTMLElement;
    private data: Array<Uint32Array>;

    constructor(container: HTMLElement | null) {
        if (!container){
            throw new Error("Could not initialize Grid since container is null");
        }

        this.contaienr = container;
    }

    load(data: Uint32Array) {
        const size = Math.sqrt(data.length) | 0;

        this.data = new Array(size).fill(0).map((_, i) => data.slice(i * size, (i + 1) * size));
    }

    private loadUI() {

    }
}