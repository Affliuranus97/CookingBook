import java.util.Comparator;
import java.util.Map;

public class RecipeComparator implements Comparator<Recipe> {
    private Map<String, Integer> query;

    RecipeComparator(Map<String, Integer> query) {
        this.query = query;
    }

    @Override
    public int compare(Recipe r1, Recipe r2) {
        return Integer.compare(r1.score(query), r2.score(query));
    }
}

