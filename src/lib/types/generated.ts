// HTTP Request Types
type HttpRequest = {
  method: Method;
  url: string;
  headers: { [key: string]: string };
  params: Array<Param>;
  body: ResponseBody | null;
  timeout: bigint | null;
};

type Method = "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS";

type Param = {
  key: string;
  value: string;
};

type RequestBody =
  | { Raw: RawBody }
  | { FormData: Array<FormDataItem> }
  | { UrlEncoded: Array<Param> }
  | { Binary: string };

type RawBody = {
  content: string;
  content_type: RawBodyType;
};

type RawBodyType = "Text" | "Json" | "Xml" | "Html" | "Javascript";

type FormDataItem = {
  key: string;
  value: FormDataValue;
};

type FormDataValue =
  | { Text: string }
  | { File: { path: string; content_type: string | null } };

// HTTP Response Types
type HttpResponse = {
  status: Status;
  headers: { [key: string]: string };
  body: ResponseBody;
  timing: Timing;
  size: Size;
};

type ResponseBody =
  | "Empty"
  | { Text: string }
  | { Json: any }
  | { Html: string }
  | { Xml: string }
  | { Binary: Array<number> };

type Size = {
  headers: number;
  body: number;
};

type Status = {
  code: number;
  text: string;
};

type Timing = {
  start: number;
  end: number;
  duration: number;
};
