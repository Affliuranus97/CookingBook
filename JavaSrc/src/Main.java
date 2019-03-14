import java.util.HashMap;
import java.util.Map;

public class Main {


    public static void main(String[] args) {
        Map<String, Integer> ingredients = new HashMap<>();
        ingredients.put("Bob", 1000);
        ingredients.put("Kufteta", 15);
        Recipe rec = new Recipe(
                "Bob s kufteta",
                "Qk bob",
                "Smesete boba s kuftetata",
                ingredients
        );

        String seralized = rec.toString();
        System.out.println(seralized);

        Recipe r = new Recipe(seralized);
        System.out.println(r);
    }

}
