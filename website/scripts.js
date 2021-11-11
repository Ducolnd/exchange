$(document).ready(function() {
    $("#transaction").submit(function(e) {
        e.preventDefault();
        let data = {
            size: parseInt(parseFloat($("#size").val())),
            price: parseInt(parseFloat($("#price").val())),
            sell: $("#sell:checked").val() === "on" ? true : false,
        }      

        data = JSON.stringify(data);

        $.ajax({
            url: 'http://127.0.0.1:8080/',
            type: 'post',
            data: data,
            headers: {"Content-Type": "application/json"},
            dataType: 'json',
            success: function (data) {
                console.info(data);
            }
        });
    })
})