use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut incoming_flash = String::new();
    for message in flash_messages.iter() {
        writeln!(incoming_flash, "<p><i>{}</i></p>", message.content()).unwrap();
    }
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta http-equiv="content-type" content="text/html; charset=utf-8">
                    <title>Publish Newsletter Issue</title>
                </head>
                <body>
                    {incoming_flash}
                    <form action="/admin/newsletters" method="post">
                        <label>Title:<br>
                            <input
                                type="text"
                                placeholder="Enter the issue title"
                                name="title"
                            >
                        </label>
                        <br>
                        <label>Plain text content:<br>
                            <textarea
                                placeholder="Enter the content in plain text"
                                name="text_content"
                                rows="20"
                                cols="50"
                            ></textarea>
                        </label>
                        <br>
                        <label>HTML content:<br>
                            <textarea
                                placeholder="Enter the content in HTML format"
                                name="html_content"
                                rows="20"
                                cols="50"
                            ></textarea>
                        </label>
                        <br>
                        <button type="submit">Publish</button>
                    </form>
                    <p><a href="/admin/dashboard">&lt;- Back</a></p>
                </body>
                </html>"#,
        )))
}
