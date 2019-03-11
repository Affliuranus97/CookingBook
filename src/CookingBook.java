import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class CookingBook {
    private List<Recipe> recipes;

    public CookingBook() {
        this.recipes = new ArrayList<>();
    }

    public void addRecipe(Recipe recipe) {
        recipes.add(recipe);
    }

    String serializeRecipes() {
        StringBuilder builder = new StringBuilder();

        for (Recipe recipe : recipes) {
            recipe.serialize(builder);
            builder.append('\n');
        }

        return builder.toString();
    }

    public void saveToFile(String path) throws IOException {
        String data = serializeRecipes();
        Files.write(Paths.get(path), data.getBytes());
    }

    public void loadFromString(String data) {
        recipes.addAll(Arrays.stream(data.split("\n")).map(Recipe::new).collect(Collectors.toList()));
    }

    public void loadFromFile(String path) throws IOException {
        loadFromString(new String(Files.readAllBytes(Paths.get(path))));
    }

    public void sortByScore(Map<String, Integer> query) {
        recipes.sort(new RecipeComparator(query));
    }

    public List<Recipe> getRecipes(Map<String, Integer> query) {
        sortByScore(query);

        List<Recipe> zeroCostRecipes = recipes.stream().filter(r -> r.score(query) == 0).collect(Collectors.toList());
        if (!zeroCostRecipes.isEmpty()) {
            return zeroCostRecipes;
        }

        return recipes;
    }

}