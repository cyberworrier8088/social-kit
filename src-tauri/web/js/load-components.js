async function loadComponent(id, file) {

    const response = await fetch(file);

    const html = await response.text();

    document.getElementById(id).innerHTML = html;

}

loadComponent("sidebar", "components/sidebar.html");

loadComponent("topbar", "components/topbar.html");

loadComponent("welcome-card", "components/welcome-card.html");