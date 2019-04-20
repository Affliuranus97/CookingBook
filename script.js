var data = [];

var unitsMap = {
    g: "Gram",
    gr: "Gram",
    гр: "Gram",
    г: "Gram",
    kg: "Kilogram",
    кг: "Kilogram",
    ml: "Milliliter",
    мл: "Milliliter",
    l: "Liter",
    л: "Liter",
    tbsp: "TableSpoon",
    t: "TableSpoon",
    сл: "TableSpoon",
    tsp: "TeaSpoon",
    чл: "TeaSpoon",
    cs: "CoffeeSpoon",
    кл: "CoffeeSpoon",
    бр: "Count"
};

var reverseUnits = {
    Gram: "гр",
    Kilogram: "кг",
    Milliliter: "мл",
    Liter: "л",
    TableSpoon: "сл",
    TeaSpoon: "чл",
    CoffeeSpoon: "кл",
    Count: "бр"
};

String.prototype.replaceAt = function(index, replacement) {
    return this.substr(0, index) + replacement + this.substr(index + replacement.length);
}
    
function unitConveter(unitType, amount){
    //Volume handle
    if (unitType == "Liter"){
        amount *= 1000;
		if (amount <= 25 && amount%5 == 0){
            return amount/5 + " чл";
        }
        if (amount <= 75 && amount%15 == 0){
            return amount/15 + " сл";
        }
        if (amount <= 10 && amount%2 == 0){
            return amount/2 + " кл";
        }
        if (amount < 1000){
            return amount + " мл";
        }
        return amount/1000 + " л";
    }  
    //Mass handle
    if (unitType == "Gram"){
		if (amount >= 1000 && amount%100 == 0){
            return amount*0.001 + " кг";
        }
        return amount + " гр";
    }   
    //Count handle
    if (unitType == "Count"){
        return amount + " бр";
    }
    return amount + " " + unitType;
}    
    
function createCard(object){
	return '<div class = "card" id = "recipe_' + object["id"] + '">' +
           '<div class = "image" style = "background-image: url(' + object["image_path"] + ')"></div>' +
           '<div class = "title">' + object["name"] + '</div></div>';
};

function createBigCards(object){
    console.log(object);
    let ingredients = "";
    for (let i of Object.entries(object["ingredients"])){
        for (let k of Object.entries(i[1])){
			ingredients += i[0] + " " + unitConveter(k[0], k[1]) + ", ";
        }
    }
    ingredients = ingredients.substr(0, ingredients.length - 2);
    return '<div class = "image" style = "background-image: url(' + object["image_path"] + ')"></div>' +
           '<div class = "title">' + object["name"] + '</div>' + 
           '<div class = "description">' + object["description"] + '</div>' +
           '<div class = "ingredients">' + ingredients + '</div>' +
           '<div class = "guide">' + object["guide"] + '</div></div>';
}

function fillCards(objects){
	let cardString = "";
    let bigCardString = "";
	for(let i = 0; i < objects.length; i++){
		cardString += createCard(objects[i]);
	}
	$(".results").html(cardString);
    
    $(".card").click(function(){
        $(".wrapper").addClass("visible");
        let id = parseInt($(this).attr('id').split("_")[1]);
        for(let i = 0; i < objects.length; i++){
            let idString = objects[i]["id"];
            if (idString == id){
                bigCardString = createBigCards(objects[i]);
                $(".recipeObject").html(bigCardString);
                break;
            }
        }
    });

    $(".wrapper").click(function(e){
        if (e.target === this) {
            $(this).removeClass("visible");
        }
    });  
}

function fillRecipeCards(objects){
    let cardString = "";
    for(let i = 0; i < objects.length; i++){
        let idString = objects[i]["id"];
        cardString += createCard(objects[i]);
    }
    $(".results").html(cardString);
}

function fillNewRecipeArray(){
    let Name = $("#recipeName").val();
    let Description = $("#descriptionArea").val();
    let Ingredients = {};
    let Guide = $("#instructionArea").val();
    let Image = $("#recipeImage").val();
    let ingredientsValue = $("#ingredientsArea").val().split(",");
    for (i in ingredientsValue){
        let ingredient = "";
        let unit = "";
        let amount = 0;
        let line = ingredientsValue[i].trim().split(" ");
        if (!isNaN(line[line.length - 1])){
            for (let j = 0; j < line.length - 1; j++){
                ingredient += line[j] + " ";
            }
            ingredient.trim();
            unit = unitsMap["бр"];
            amount = line[line.length - 1];
        }
        else {
            for (let j = 0; j < line.length - 2; j++){
                ingredient += line[j] + " ";
            }
            ingredient.trim();
            unit = unitsMap[line[line.length - 1]];
            amount = line[line.length - 2];
        }            
        Ingredients[ingredient] = {};
        Ingredients[ingredient][unit] = parseInt(amount);
    }
    let newRecipe = {
        id: 0,
        name: Name,
        description: Description,
        guide: Guide,
        image_path: Image,
        ingredients: Ingredients
    };
    return newRecipe;
}


$("#srchBar").on('keyup', function(e){
    data = [];
    fillCards(data);
    let string = $(this).val();
    let ingredientArray = string.split(",");
    let resultString = "";
    for (i in ingredientArray){
        let singleIngredient = "";
        let parts = ingredientArray[i].split(" ");
        for (let k = 0; k < (parts.length - 2); k++){
            singleIngredient += parts[k] + " ";
        }
        singleIngredient = singleIngredient.trim();
        singleIngredient += "/";
        singleIngredient += parts[parts.length - 2];
        singleIngredient += "_";
        singleIngredient += unitsMap[parts[parts.length - 1]];
        resultString += singleIngredient + "/";
    }
    resultString = resultString.substr(0, resultString.length - 1);
    console.log(resultString);
    $.ajax({
        url: "api/search/" + resultString,
        method: "GET",
    })
    .then(function(info, status){
        console.log(status);
        console.log(info);
        data = info;
        fillCards(data);
    })
});

$("#add").click(function(e){
    let newRecipe = fillNewRecipeArray();
    let jsonLog = JSON.stringify(newRecipe);
    console.log(jsonLog);
    $.ajax({
        method: "POST",
        url: "api/add_recipe",
        data: jsonLog,
        dataType: "json",
        contentType: "application/json; charset=utf-8",
        failure: function(error){
            console.log(error);
        },
        success: function(data){
            console.log(data);
            window.alert("Recipe successfully added!");
            $("#recipeName").val('');
            $("#descriptionArea").val('');
            $("#ingredientsArea").val('');
            $("#instructionArea").val('');
            $("#recipeImage").val('');
        }
    })
});