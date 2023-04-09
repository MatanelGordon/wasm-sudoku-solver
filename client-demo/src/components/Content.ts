import { ComponentBase } from "../core";

export class Content extends ComponentBase<HTMLSpanElement> {
  #value: number;
  constructor() {
    super("span");

    this.#value = 0;

    this.element.style.pointerEvents = "none";
  }

  get value() {
    return this.#value;
  }

  set value(value) {
    this.#value = value;
    this.element.textContent = value.toString();
  }
}
