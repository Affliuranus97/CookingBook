import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Recipe {
    private String name;
    private String description;
    private String instructions;
    private Map<String, Integer> ingredients;

    public Recipe(String name, String description, String instructions, Map<String, Integer> ingredients) {
        this.name = name;
        this.description = description;
        this.instructions = instructions;
        this.ingredients = ingredients;
    }

    public Recipe(String serialized) {
        ingredients = new HashMap<>();
        deserialize(serialized);
    }

    public int score(Map<String, Integer> query) {
        int result = 0;

        for (Map.Entry<String, Integer> element : ingredients.entrySet()) {
            int recipeNeeds = element.getValue();
            int excess = query.get(element.getKey()) - recipeNeeds;

            if (excess < 0) {
                result += excess;
            }
        }

        return result; // 0, -neshto
    }


    private static void serializeIngredient(StringBuilder builder, String name, int amount) {
        builder.append(name).append(' ').append(amount);
    }

    public String serialize(StringBuilder builder) {
        builder.append(name);
        builder.append(';');
        builder.append(description);
        builder.append(';');
        builder.append(instructions);
        builder.append(';');
        for (Map.Entry<String, Integer> ingredient : ingredients.entrySet()) {
            serializeIngredient(builder, ingredient.getKey(), ingredient.getValue());
            builder.append(',');
        }
        builder.deleteCharAt(builder.length() - 1);
        return builder.toString();
    }

    public void deserialize(String serialized) {
        String[] result = serialized.split(";");
        name = result[0];
        description = result[1];
        instructions = result[2];

        for (String i : result[3].split(",")) {
            int lastSpace = i.lastIndexOf(' ');
            String name = i.substring(0, lastSpace);
            int amount = Integer.valueOf(i.subSequence(lastSpace + 1, i.length()).toString());

            ingredients.put(name, amount);
        }
    }

    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder();
        serialize(builder);
        return builder.toString();
    }
}
