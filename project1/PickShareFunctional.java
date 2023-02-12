package project1;

import java.util.stream.Stream;

public class PickShareFunctional {
    public static ShareInfo findHighPriced(Stream<String> shares) {
        return shares.map(share -> ShareUtil.getPrice(share))
                .filter(shareInfo -> ShareUtil.isPriceLessThan(500).test(shareInfo))
                .max((share1, share2) -> share1.price.compareTo(share2.price)).get();
    }
}
