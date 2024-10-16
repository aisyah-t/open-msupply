import { calculateStockAtRisk } from "./convert_data";
import {  calculateQuantity } from "./convert_data";
import inputData from "./input.json" assert { type: "json" };
import outputData from "./output.json" assert { type: "json" };

// describe("test process stock lines", () => {
//   beforeAll(() => {
//     jest.useFakeTimers();
//     jest.setSystemTime(new Date("2024-04-01"));
//   });
//   // mock out the 4 internal functions
//   // calculateExpectedUsage.mockImplementation(() => 20);
//   // calculateStockAtRisk.mockImplementation(() => 10);
//   // roundDaysToInteger.mockImplementation(() => 10);
//   // it("end to end", () => {
//   //   const result = processStockLines(inputData.stockLines.nodes);
//   //   expect(result).toEqual(outputData.stockLines.nodes);
//   // });
//   afterAll(() => {
//     jest.useRealTimers();
//   });
// });

describe("Adds monthlyConsumption correctly from query result", () => {
  it("returns undefined if either are undefined", () => {
    expect(calculateQuantity(undefined, "id")).toBe(undefined);
    expect(calculateQuantity(inputData.thisMonthConsumption, undefined)).toBe(undefined);

  });
  it("returns month consumption if available", () => {
    expect(calculateQuantity(inputData.thisMonthConsumption, "101")).toBe(200);
  });
  it("returns undefined if undefined", () => {
    expect(calculateQuantity(inputData.thisMonthConsumption, "non existent id")).toBe(undefined)
  });
});

describe("Adds lastMonthConsumption correctly from query result", () => {
  it("returns undefined if either are undefined", () => {
    expect(calculateQuantity(undefined, "id")).toBe(undefined);
    expect(calculateQuantity(inputData.lastMonthConsumption, undefined)).toBe(undefined);

  });
  it("returns month consumption if available", () => {
    expect(calculateQuantity(inputData.lastMonthConsumption, "101")).toBe(500);
  });
  it("returns undefined if undefined", () => {
    expect(calculateQuantity(inputData.lastMonthConsumption, "non existent id")).toBe(undefined)
  });
});

describe("Adds twoMonthsAgoConsumption correctly from query result", () => {
  it("returns undefined if either are undefined", () => {
    expect(calculateQuantity(undefined, "id")).toBe(undefined);
    expect(calculateQuantity(inputData.twoMonthsAgoConsumption, undefined)).toBe(undefined);

  });
  it("returns month consumption if available", () => {
    expect(calculateQuantity(inputData.twoMonthsAgoConsumption, "102")).toBe(421);
  });
  it("returns undefined if undefined", () => {
    expect(calculateQuantity(inputData.twoMonthsAgoConsumption, "non existent id")).toBe(undefined)
  });
});

describe("Adds expiringInSixMonths correctly from query result", () => {
  it("returns undefined if either are undefined", () => {
    expect(calculateQuantity(undefined, "id")).toBe(undefined);
    expect(calculateQuantity(inputData.expiringInSixMonths, undefined)).toBe(undefined);
  });
  it("returns month consumption if available", () => {
    expect(calculateQuantity(inputData.expiringInSixMonths, "102")).toBe(75);
  });
  it("returns undefined if undefined", () => {
    expect(calculateQuantity(inputData.expiringInSixMonths, "non existent id")).toBe(undefined)
  });
});

describe("Adds expiringIntwelveMonths correctly from query result", () => {
  it("returns undefined if either are undefined", () => {
    expect(calculateQuantity(undefined, "id")).toBe(undefined);
    expect(calculateQuantity(inputData.expiringInTwelveMonths, undefined)).toBe(undefined);
  });
  it("returns month consumption if available", () => {
    expect(calculateQuantity(inputData.expiringInTwelveMonths, "102")).toBe(92);
  });
  it("returns undefined if undefined", () => {
    expect(calculateQuantity(inputData.expiringInTwelveMonths, "non existent id")).toBe(undefined)
  });
});

describe("Adds AMC12 correctly from query result", () => {
  it("returns undefined if either are undefined", () => {
    expect(calculateQuantity(undefined, "id")).toBe(undefined);
    expect(calculateQuantity(inputData.AMCTwelve, undefined)).toBe(undefined);
  });
  it("returns month consumption if available", () => {
    expect(calculateQuantity(inputData.AMCTwelve, "102")).toBe(92.4);
  });
  it("returns undefined if undefined", () => {
    expect(calculateQuantity(inputData.AMCTwelve, "non existent id")).toBe(undefined)
  });
});

describe("Adds AMC24 correctly from query result", () => {
  it("returns undefined if either are undefined", () => {
    expect(calculateQuantity(undefined, "id")).toBe(undefined);
    expect(calculateQuantity(inputData.AMCTwentyFour, undefined)).toBe(undefined);
  });
  it("returns month consumption if available", () => {
    expect(calculateQuantity(inputData.AMCTwentyFour, "102")).toBe(192.4);
  });
  it("returns undefined if undefined", () => {
    expect(calculateQuantity(inputData.AMCTwentyFour, "non existent id")).toBe(undefined)
  });
});