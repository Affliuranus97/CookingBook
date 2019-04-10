var data = [];

$("#srchBar").on('input', function(e){
    $.ajax({
        url: "api/search/" + ($(this).val()),
        method: "GET",
    })
    .then(function(data, status){
        console.log(status);
        console.log(data);
        fillCards(data);
    })
});
    
function createCard(object){
	return '<div class="card" id="recipe_' + object["id"] + '"><div class="image"></div>' +
	'<div class="title">' + object["name"] + '</div></div>';
};

function fillCards(objects){
	let cardString = "";
	for(let i = 0; i < objects.length; i++){
		cardString += createCard(objects[i]);
	}
	$(".results").html(cardString);
}

function fillRecipeCards(objects){
    let cardString = "";
    for(let i = 0; i < objects.length; i++){
        let idString = objects[i]["id"];
        cardString += createCard(objects[i]);
    }
    $(".results").html(cardString);
}

$(".card").click(function(){
	$(".wrapper").addClass("visible");
});

$(".wrapper").click(function(e){
	if (e.target === this) {
		$(this).removeClass("visible");
	}
});