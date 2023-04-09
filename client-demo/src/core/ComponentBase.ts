export abstract class ComponentBase<P extends HTMLElement = HTMLElement> {
  protected readonly element: P;

  constructor(tagName: string) {
    this.element = document.createElement(tagName) as P;
  }

  get classList() {
    return this.element.classList;
  }

  setAttribute(key: string, value: string | number | boolean) {
    this.element.setAttribute(key, value.toString());
  }

  setDataSet(dataset: Record<string, string>) {
    Object.entries(dataset).forEach(([key, value]) => {
      this.setAttribute(`data-${key}`, value);
    });
  }

  getData(key: string) {
    return this.element.dataset[`data-${key}`];
  }

  getElement() {
    return this.element;
  }

  mount(container: HTMLElement) {
    container.appendChild(this.element);
  }
}
