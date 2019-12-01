$(".searchList").click(function(){
    $(this).find(".arrow").toggleClass("rotateImage");
    if($("#priceButton").find(".arrow").hasClass("rotateImage")){
        $("#hidden").removeClass("hidden");
    }
    else{
        $("#hidden").addClass("hidden");
    }
});


var rangeSlider = document.getElementById("slideBar");
var rangeBullet = document.getElementById("priceValue");
rangeBullet.innerHTML = rangeSlider.value;

rangeSlider.oninput = function(){
  rangeBullet.innerHTML = this.value;
  var bulletPosition = (rangeSlider.value /rangeSlider.max);
  rangeBullet.style.left = (bulletPosition * 278) + "px";
};