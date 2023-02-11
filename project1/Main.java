package project1;

import java.util.Optional;

public class Main {
    public static void main(String[] args) {
        Optional<ShareInfo> highShare = PickShareFunctional.findHighPriced(Shares.symbols.stream());
        if (highShare.isPresent()) {
            System.out.println("High priced uder $500 is " + highShare.get());
        } else {
            System.out.println("No High priced uder $500 exist");
        }
    }
}