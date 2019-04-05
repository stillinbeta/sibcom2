var currentTheme;

document.addEventListener("DOMContentLoaded", function() {
    currentTheme = document.body.className;

    var themes = ["solarized-dark", "solarized-light", "terminal"];

    var buttons = themes.map(theme => {
        let div = document.createElement('div');
        div.className = theme + " theme-preview";
        div.role = "button";
        div.onmouseenter =  function() {
            document.body.className = theme;
        };
        div.onclick = function() {
            currentTheme = theme;
            document.cookie = "theme="+theme;
        };

        div.appendChild(document.createTextNode("{"));
        return div;
    });

    var wrapper = document.createElement('div');
    wrapper.className = "theme-previews";
    wrapper.onmouseleave = function() {
        document.body.className = currentTheme;
    };
    buttons.map(b => wrapper.appendChild(b));

    document.body.appendChild(wrapper);
});
