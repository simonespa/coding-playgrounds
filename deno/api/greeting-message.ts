import { Context } from "https://deno.land/x/oak@v11.1.0/context.ts";

export default ({ response }: Context) => {
  response.body = {
    message: "This is a greeting",
  };
};
