package project1;

import java.util.Optional;

public class Main {
    public static void main(String[] args) {
        long startTime = System.currentTimeMillis();

        Optional<ShareInfo> highShare = PickShareFunctional.findHighPriced(Shares.symbols.stream());

        long endTime1 = System.currentTimeMillis();

        // Optional<ShareInfo> highShareParallel = PickShareFunctional.findHighPriced(Shares.symbols.parallelStream());
        // long endTime2 = System.currentTimeMillis();

        long duration1 = (endTime1 - startTime);
        if (highShare.isPresent()) {
            System.out.println("High priced uder $500 is " + highShare.get());
        } else {
            System.out.println("No High priced uder $500 exist");
        }
        System.out
                .println("Time execution of findHighPriced(Shares.symbols.stream()) = " + duration1 + " milliseconds");

        // long duration2 = (endTime2 - endTime1);
        // if (highShareParallel.isPresent()) {
        //     System.out.println("High priced uder $500 is " + highShareParallel.get());
        // } else {
        //     System.out.println("No High priced uder $500 exist");
        // }
        // System.out.println(
        //         "Time execution of findHighPriced(Shares.symbols.parallelStream()) = " + duration2 + " milliseconds");

    }
}