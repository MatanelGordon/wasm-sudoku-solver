import { PositionalComponentBase } from "../core";
import { Content } from "./Content";

export class Cell extends PositionalComponentBase<HTMLDivElement> {
  #contentElement: Content;

  constructor() {
    super("div");
    this.classList.add("cell");

    this.#contentElement = new Content();
    this.#contentElement.classList.add("content");

    this.#contentElement.mount(this.element);
  }

  get value() {
    return this.#contentElement.value;
  }

  set value(value: number) {
    this.#contentElement.value = value;
    this.setAttribute("data-value", value);
  }
}
