use resuma::prelude::*;

use crate::stats::grouped;

pub fn page(req: FlowRequest) -> View {
    set_page_title("Admin | PlacaQR");
    set_page_description("Private PlacaQR statistics.");
    set_page_robots("noindex, nofollow");
    set_page_lang("es");
    if !crate::site::request_is_admin() {
        return login_view(req.query_param("e").is_some());
    }
    dash_view()
}

fn login_view(failed: bool) -> View {
    let notice = if failed {
        view! {
            <p class="hint admin-error" role="alert">
                "Ese usuario o clave no coincide."
            </p>
        }
    } else {
        view! { <span hidden=""></span> }
    };
    view! {
        <main class="content-section admin-page">
            <p class="eyebrow">"PlacaQR"</p>
            <h1>"Admin"</h1>
            <p class="hero-lead">
                "Las descargas terminadas, por formato y por forma, están en esta página."
            </p>
            {notice}
            <Form submit={crate::actions::admin_login} class="hero-form admin-login">
                <label>
                    "Usuario"
                    <input name="user" type="text" autocomplete="username" required=true />
                </label>
                <label>
                    "Clave"
                    <input name="password" type="password" autocomplete="current-password" required=true />
                </label>
                <button type="submit" class="btn btn-primary">"Entrar"</button>
            </Form>
        </main>
    }
}

fn dash_view() -> View {
    let activity = crate::stats::snapshot();
    view! {
        <main class="content-section admin-page">
            <div class="admin-head">
                <div>
                    <p class="eyebrow">"PlacaQR"</p>
                    <h1>"Estadísticas"</h1>
                </div>
                <Form submit={crate::actions::admin_logout}>
                    <input type="hidden" name="ok" value="1" />
                    <button type="submit" class="btn">"Cerrar sesión"</button>
                </Form>
            </div>
            <h2>"Formatos"</h2>
            <div class="admin-grid">
                {stat_card("3MF", grouped(activity.three_mf))}
                {stat_card("STL", grouped(activity.stl))}
                {stat_card("PNG", grouped(activity.png))}
                {stat_card("SVG", grouped(activity.svg))}
            </div>
            <h2>"Formas"</h2>
            <div class="admin-grid">
                {stat_card("Soporte", grouped(activity.stand))}
                {stat_card("Baldosa", grouped(activity.tile))}
                {stat_card("Llavero", grouped(activity.keychain))}
                {stat_card("Placa", grouped(activity.plaque))}
                {stat_card("Moneda", grouped(activity.coin))}
            </div>
            <p class="hint">
                "Cada número es una descarga que terminó. No se publica."
            </p>
        </main>
    }
}

fn stat_card(label: &'static str, value: String) -> View {
    view! {
        <article class="admin-stat">
            <p class="admin-stat-value">{value}</p>
            <p class="hint">{label}</p>
        </article>
    }
}
