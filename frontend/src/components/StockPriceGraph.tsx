import React from "react";
import { Line } from "react-chartjs-2";
import "chart.js/auto"; // Necessary for Chart.js v3

const StockPriceGraph = ({
  labels,
  dataPoints,
}: {
  labels: string[];
  dataPoints: number[];
}) => {
  const data = {
    labels: labels,
    datasets: [
      {
        label: "Stock Price",
        data: dataPoints,
        fill: false,
        backgroundColor: "rgb(75, 192, 192)",
        borderColor: "rgba(75, 192, 192, 0.2)",
      },
    ],
  };

  const options = {
    responsive: true,
    plugins: {
      legend: {
        position: "top" as const,
      },
      title: {
        display: true,
        text: "stock price",
      },
    },
  };

  return <Line options={options} data={data} />;
};

export default StockPriceGraph;
