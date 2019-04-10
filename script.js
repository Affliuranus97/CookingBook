var data = [];

String.prototype.replaceAt=function(index, replacement) {
    return this.substr(0, index) + replacement + this.substr(index + replacement.length);
}
    
function createCard(object){
	return '<div class = "card" id = "recipe_' + object["id"] + '"><div class = "image"></div>' +
	'<div class = "title">' + object["name"] + '</div></div>';
};

function createBigCards(object){
    let ingredients = "";
    for (let i of Object.entries(object)){
        ingredients += object["ingredients"];
    }
    return '<div class = "image"></div>' +
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
        bigCardString = createBigCards(objects);
        $(".recipeObject").html(cardString);
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


$("#srchBar").on('keyup', function(e){
    let string = $(this).val();
    let i = string.indexOf(" ");
    if (i == -1){
        console.log("not space");
        fillCards(data);
        return;
    }
    string = string.replaceAt(i, "/");
    i = string.indexOf(" ");
    if (i == -1){
        console.log("not space");
        fillCards(data);
        return;
    }
    string = string.replaceAt(i, "_");
    $.ajax({
        url: "api/search/" + string,
        method: "GET",
    })
    .then(function(info, status){
        console.log(status);
        console.log(info);
        data = info;
        fillCards(data);
    })
});