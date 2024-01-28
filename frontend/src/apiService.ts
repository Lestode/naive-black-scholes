const BASE_URL = ""; //current base_url is empty since everything is redirected through the proxy

export const fetchBestMatches = async (symbol: string): Promise<string[]> => {
  try {
    const response = await fetch(`${BASE_URL}/get_best_matches/${symbol}`);
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    const data: string[] = await response.json();
    return data;
  } catch (error) {
    throw error;
  }
};

export const fetchLastYearPrices = async (
  symbol: string
): Promise<LastYearPrices> => {
  try {
    const response = await fetch(`${BASE_URL}/get_last_year_prices/${symbol}`);
    if (!response.ok) {
      throw new Error("Network response was not ok");
    }
    const data: LastYearPrices = await response.json();
    return data;
  } catch (error) {
    throw error;
  }
};

interface LastYearPrices {
  months: number[];
  prices: string[];
}
