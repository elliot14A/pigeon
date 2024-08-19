import type * as types from "../types";

class MethodClass {
  method = $state("");
}

class UrlClass {
  url = $state("");
  setUrl(newUrl: string) {
    this.url = newUrl;
  }
}

class HeadersClass {
  headers = $state<types.KeyValuePair[]>([{ key: "", value: "" }]);

  addHeaders() {
    this.headers.push({ key: "", value: "" });
  }

  removeHeader(index: number) {
    this.headers.filter((_, i) => i !== index);
  }
}

class ParamsClass {
  params = $state<types.KeyValuePair[]>([{ key: "", value: "" }]);

  addParams() {
    this.params.push({ key: "", value: "" });
  }

  removeParams(index: number) {
    this.params = this.params.filter((_, i) => i !== index);
  }
}

class BodyClass {
  body = $state("");
}

export const requestBody = new BodyClass();
export const requestParams = new ParamsClass();
export const requestHeaders = new HeadersClass();
export const requestUrl = new UrlClass();
export const requestMethod = new MethodClass();
