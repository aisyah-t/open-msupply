function convert_data() {
  const res = JSON.parse(Host.inputString());
  res.items.nodes = processItemLines(res);
  Host.outputString(JSON.stringify(res));
}

const processItemLines = (res) => {
  res.items.nodes.forEach((item) => {
    item.monthConsumption = calculateQuantity(res.thisMonthConsumption, item.id);
    item.lastMonthConsumption = calculateQuantity(
      res.lastMonthConsumption,
      item.id
    );
    item.twoMonthsAgoConsumption = calculateQuantity(
      res.twoMonthsAgoConsumption,
      item.id
    );
    item.expiringInSixMonths = calculateQuantity(
      res.expiringInSixMonths,
      item.id
    );
    item.expiringInTwelveMonths = calculateQuantity(
      res.expiringInTwelveMonths,
      item.id
    );
    item.stockOnOrder = calculateQuantity(res.stockOnOrder, item.id);
    item.AMCTwelve = calculateQuantity(res.AMCTwelve, item.id);
    item.AMCTwentyFour = calculateQuantity(res.AMCTwentyFour, item.id);
    item.SOH = calculateStatValue(item.stats.availableStockOnHand);
    item.MOS = calculateStatValue(item.stats.availableMonthsOfStockOnHand);
  });
  return res.items.nodes;
};

// function adds month consumption to data  (either this or last month)
const calculateQuantity = (queryResult, id) => {
  let quantity = 0;
  if (!!queryResult && !!id) {
    const node = queryResult.find((element) => element.item_id == id);
    quantity = node?.quantity ? node.quantity : 0;
  }
  return quantity;
};

const calculateStatValue = (value) => {
  let returnValue = 0;
  if (!!value) {
    // round to 1 decimal
    returnValue = Math.round(value * 10) / 10;
  }
  return returnValue;
};

module.exports = {
  convert_data,
  calculateQuantity,
  calculateStatValue,
  processItemLines,
};
