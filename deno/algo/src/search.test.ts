import { assertEquals } from "https://deno.land/std@0.168.0/testing/asserts.ts";
import { linearSearch, binarySearch } from "./search.ts";

Deno.test("Linear Search", () => {
  const sequence = [5, 4, 3, 2, 1];
  const found = linearSearch(3, sequence);
  assertEquals(found, true);
});

Deno.test(function binarySearch() {
  const sequence = [5, 4, 3, 2, 1];
  const found = linearSearch(4, sequence);
  assertEquals(found, true);
});
