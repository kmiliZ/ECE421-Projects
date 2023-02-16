package project1;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.math.BigDecimal;
import java.net.URL;
import java.net.URLConnection;

public class APIFinance {
    private static final String BASE_URL = "https://www.alphavantage.co/query?";
    private final static String apiKey = "O0IORAI4LIZIK61W";

    private static final long timeLimit = 60000;

    public static BigDecimal getPrice(final String symbol, Boolean isRetry) {
        // sleepForLimit();
        Boolean rFlag = isRetry == null ? false : isRetry;
        BigDecimal price = new BigDecimal(0);

        try {
            URL url = new URL(BASE_URL +
                    "function=GLOBAL_QUOTE&symbol=" + symbol + "&apikey=" + apiKey);
            URLConnection connection = url.openConnection();
            InputStreamReader inputStream = new InputStreamReader(connection.getInputStream(), "UTF-8");
            BufferedReader bufferedReader = new BufferedReader(inputStream);
            String line;
            while ((line = bufferedReader.readLine()) != null) {

                if (line.contains("price")) {
                    price = new BigDecimal(line.split("\"")[3].trim());
                } else if (line.contains(
                    "Our standard API call frequency is 5 calls per minute and 500 calls per day.")) {
                    if (!rFlag) {
                        try {
                            Thread.sleep(timeLimit);
                        } catch (InterruptedException e) {
                            e.printStackTrace();
                        }
                        price = getPrice(symbol, true);
                    } else {
                        System.out.println("Exceeded API call limitation for today!!!");
                        break;
                    }
                } else if (line.contains("{}")) {
                    System.out.println(symbol + " was not found in the API!!!");
                    break;
                }
            }
            bufferedReader.close();
        } catch (IOException e) {
            System.out.println("failure sending request");
        }

        return price;
    }
}
