import { Context } from "https://deno.land/x/oak@v11.1.0/context.ts";

export default async ({ response }: Context, next: () => Promise<unknown>) => {
  try {
    await next();
  } catch (error) {
    response.status = 500;
    response.body = { msg: error.message };
  }
};
