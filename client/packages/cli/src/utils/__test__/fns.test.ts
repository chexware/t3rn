import { describe, expect, test } from "@jest/globals"
import { z } from "zod"
import { validate } from "../fns.ts"

const Schema = z.object({
  tokenId: z.number(),
  tokenSymbol: z.string(),
  tokenDecimals: z.number(),
})

describe("validate", () => {
  test("should pass validation if a valid value is passed", () => {
    const value = { tokenId: 123, tokenSymbol: "ABC", tokenDecimals: 18 }
    expect(validate(Schema, value)).not.toBeUndefined()
    expect(validate(Schema, value)).toEqual(value)
  })

  test("should fail validation if an invalid value is passed", () => {
    const value = { tokenId: "123", tokenSymbol: "ABC", tokenDecimals: 18.5 }
    console.log = jest.fn()
    expect(validate(Schema, value)).toBeUndefined()
    expect(console.log).toHaveBeenCalledWith(expect.stringContaining("ERROR"))
  })
})
