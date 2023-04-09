import { ComponentBase } from "./ComponentBase";

export class PositionalComponentBase<
  T extends HTMLElement = HTMLElement
> extends ComponentBase<T> {
  #row: number = 0;
  #col: number = 0;

  get col() {
    return this.#col;
  }

  set col(value: number) {
    this.#col = value;
    this.setAttribute("data-col", value);
  }

  get row() {
    return this.#row;
  }

  set row(value: number) {
    this.#row = value;
    this.setAttribute("data-row", value);
  }
}
