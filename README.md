# Duri #

CLI tool to get data URI of a file.

## Usage

- Read from file path:

  ```console
  duri image.png
  ```

- Read from standard input:

  ```console
  echo image.png | duri -
  ```

## Use cases

Assume that you need to upload file to a RESTful HTTP API. The HTTP API may require posted data to be JSON string and the file content to be in form of base64-encoding [data URI](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs).

You can combine Duri with [jo](https://github.com/jpmens/jo) to build JSON, like:

```console
jo -d. file.name=image.png file.content=$(duri image.png)
```

then pass to a CLI HTTP client, like [HTTPie](https://httpie.io/):

```sh
jo -d. file.name=image.png file.content=$(duri image.png) | http example-api.vn/ekyc/
```

The `duri` + `jo` combo will generate a JSON like

```json
{
  "file": {
    "name": "image.png",
    "content": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAAAAAA6fptVAAABI2..."
  }
}

```

The string is passed to HTTPie via standard input and HTTPie will build a POST request with that JSON data.

Note that, if your HTTP API requires file to be in plain base64 string, not beginning with `data:xxx`, you don't need Duri.
In that case, just use `jo` alone, with its `%` directive:

```sh
jo -d. file.name=image.png file.content=%image.png | http example-api.vn/ekyc/
```
