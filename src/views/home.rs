use std::sync::Arc;

use base64::Engine;
use dioxus::prelude::*;
use futures_util::StreamExt;
use manrex::{model::{chapter::ChapterFilter, manga::MangaInclude}, ChapterId, Client};
use tokio::sync::Mutex;

#[component]
pub fn Home() -> Element {
    let id = "6cf34aaa-0799-48b6-a392-dcc5b1c9b8fc";
    let client = use_context::<Arc<Mutex<Client>>>();

    let mut hdef = use_signal(|| false);

    let mut manga = use_signal(|| None);
    let mut chapters = use_signal(Vec::new);
    let mut cover_art = use_signal(|| None);
    let mut chapter: Signal<Option<ChapterId>> = use_signal(|| None);
    let mut pages = use_signal(Vec::new);

    let c = client.clone();
    use_future(move || {
        let client = c.clone();
        async move {
            if let Ok(m) = client.lock().await.get_manga(id, [MangaInclude::CoverArt]).await {
                manga.set(Some(m));
            }

            if let Ok(c) = client
                .lock()
                .await
                .list_chapters(ChapterFilter::default().manga(id).limit(50))
                .await {

                if let Some(first) = c.data.first() {
                    chapter.set(Some(first.id.clone()));
                }

                chapters.set(c.data);
            }
        }
    });

    let _ = use_resource(move || {
        async move {
            if let Some(manga) = manga.read().as_ref() {
                let (_mime, mut stream) = manga.get_cover_art(None)?.fetch().await?;
                let mut content = Vec::new();

                while let Some(Ok(chunk)) = stream.next().await {
                    content.extend(chunk);
                }
                cover_art.set(Some(base64::engine::general_purpose::STANDARD.encode(&content)));
            }
            Ok::<(), manrex::Error>(())
        }
    });

    let c = client.clone();
    let _ = use_resource(move || {
        let client = c.clone();
        async move {
            if let Some(chapter) = chapter.read().as_ref() {
                let server = client.lock().await.get_at_home_server(chapter, false).await?;
                pages.set(Vec::new());

                // PREF: Create threads that fetch images in parallel
                for image in if *hdef.read() { server.saver_images() } else { server.images() }.iter() {
                    let (_mime, mut stream) = image.fetch().await?;
                    {
                        let mut content = Vec::new();

                        // Stream chunks of bytes from the image response to the file
                        while let Some(Ok(chunk)) = stream.next().await {
                            content.extend(chunk);
                        }

                        pages.write().push(base64::engine::general_purpose::STANDARD.encode(&content))
                    }
                }
            }
            Ok::<(), manrex::Error>(())
        }
    });

    rsx! {
        div {
            class: "flex min-h-full w-full",
            aside {
                class: "border-r border-white relative min-h-full bg-zinc-900 text-white",
                ul {
                    class: "sticky top-0 overflow-y-auto px-4 py-2 max-h-[calc(100vh-1.5rem)]",
                    div {
                        class: "flex gap-1",
                        input { 
                            r#type: "checkbox",
                            oninput: move |evt| {
                                let is_enabled = evt.value() == "true";
                                hdef.set(is_enabled);
                            }
                        }
                        label { "High Resolution" }
                    }
                    for (i, c) in chapters.read().clone().into_iter().enumerate() {
                        li { 
                            button {
                                onclick: move |_e| {
                                    chapter.set(Some(c.id.clone()))
                                },
                                "Chapter {i}"
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col items-center bg-zinc-900 min-h-full w-full",
                if let Some(cover_art) = cover_art.as_ref() {
                    div {
                        class: "w-full h-[calc(100vh-1.5rem)] flex justify-center items-center",
                        img {
                            id: "cover",
                            class: "max-w-[800px] max-h-full",
                            src: format!("data:image/png;base64,{cover_art}"),
                            title: manga.read().as_ref().and_then(|m| m.attributes.title.get("en").cloned()),
                        }
                    }
                }
                for (i, page) in pages.iter().enumerate() {
                    img {
                        id: format!("page-{i}"),
                        key: "{i}",
                        class: "max-w-[800px] w-full",
                        src: format!("data:image/png;base64,{page}"),
                    }
                }
            }
        }
    }
}
