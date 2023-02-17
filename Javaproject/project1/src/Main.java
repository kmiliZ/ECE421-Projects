import java.util.Optional;

public class Main {
    public static void main(String[] args) {
        long startTime = System.currentTimeMillis();

        Optional<ShareInfo> highShare = PickShareFunctional.findHighPriced(Shares.symbols.stream());

        // Optional<ShareInfo> highShare =
        // PickShareFunctional.findHighPriced(Shares.symbols.parallelStream());
        long endTime = System.currentTimeMillis();

        long duration = (endTime - startTime);
        if (highShare.isPresent()) {
            System.out.println("High priced under $500 is " + highShare.get());
        } else {
            System.out.println("No High priced under $500 exist");
        }
        System.out
                .println("Time execution of findHighPriced = " + duration + " milliseconds");
    }
}