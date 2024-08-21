import type * as types from "../types";

export class RequestClass {
  method = $state("");
  url = $state("");
  headers = $state<types.KeyValuePair[]>([{ key: "", value: "" }]);
  params = $state<types.KeyValuePair[]>([{ key: "", value: "" }]);
  body = $state("");

  setUrl(newUrl: string) {
    this.url = newUrl;
  }

  addHeaders() {
    this.headers.push({ key: "", value: "" });
  }

  removeHeader(index: number) {
    this.headers = this.headers.filter((_, i) => i !== index);
  }

  addParams() {
    this.params.push({ key: "", value: "" });
  }

  removeParams(index: number) {
    this.params = this.params.filter((_, i) => i !== index);
  }
}

export const request = new RequestClass();
