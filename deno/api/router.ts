import { Router } from "https://deno.land/x/oak/mod.ts";;
import getGreetingMessage from "./greeting-message.ts";

const router = new Router();

router
  .get("/greetings", getGreetingMessage)

export default router;
