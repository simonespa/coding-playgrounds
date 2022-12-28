import { Context } from "https://deno.land/x/oak@v11.1.0/context.ts";

export default ({ response }: Context) => {
  response.status = 404;
  response.body = { msg: "Not Found" };
};
