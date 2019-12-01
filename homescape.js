$(".searchList").click(function(){
    $(this).find(".arrow").toggleClass("rotateImage");
    if($("#priceButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenPrice").removeClass("hidden");
    }
    else{
        $("#hiddenPrice").addClass("hidden");
    }
    if($("#sizeButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenSize").removeClass("hidden");
    }
    else{
        $("#hiddenSize").addClass("hidden");
    }
    if($("#kindButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenKind").removeClass("hidden");
    }
    else{
        $("#hiddenKind").addClass("hidden");
    }    
    if($("#typeButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenType").removeClass("hidden");
    }
    else{
        $("#hiddenType").addClass("hidden");
    }    
    if($("#constButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenConst").removeClass("hidden");
    }
    else{
        $("#hiddenConst").addClass("hidden");
    }  
    if($("#yearButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenYear").removeClass("hidden");
    }
    else{
        $("#hiddenYear").addClass("hidden");
    }   
    if($("#roomButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenRoom").removeClass("hidden");
    }
    else{
        $("#hiddenRoom").addClass("hidden");
    }  
    if($("#floorButton").find(".arrow").hasClass("rotateImage")){
        $("#hiddenFloor").removeClass("hidden");
    }
    else{
        $("#hiddenFloor").addClass("hidden");
    }
});


var priceSlider = document.getElementById("slideBarPrice");
var priceBullet = document.getElementById("priceValue");
priceBullet.innerHTML = priceSlider.value;

priceSlider.oninput = function(){
  priceBullet.innerHTML = this.value;
  var bulletPosition = (priceSlider.value /priceSlider.max);
  priceBullet.style.left = (bulletPosition * 278) + "px";
};

var sizeSlider = document.getElementById("slideBarSize");
var sizeBullet = document.getElementById("sizeValue");
sizeBullet.innerHTML = sizeSlider.value;

sizeSlider.oninput = function(){
  sizeBullet.innerHTML = this.value;
  var bulletPosition = (sizeSlider.value /sizeSlider.max);
  sizeBullet.style.left = (bulletPosition * 278) + "px";
};

var yearSlider = document.getElementById("slideBarYear");
var yearBullet = document.getElementById("yearValue");
yearBullet.innerHTML = yearSlider.value;

yearSlider.oninput = function(){
  yearBullet.innerHTML = this.value;
  var bulletPosition = (yearSlider.value /yearSlider.max);
  yearBullet.style.left = (bulletPosition * 278) + "px";
};

var floorSlider = document.getElementById("slideBarFloor");
var floorBullet = document.getElementById("floorValue");
floorBullet.innerHTML = floorSlider.value;

floorSlider.oninput = function(){
  floorBullet.innerHTML = this.value;
  var bulletPosition = (floorSlider.value /floorSlider.max);
  floorBullet.style.left = (bulletPosition * 278) + "px";
};