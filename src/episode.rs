use serde::{Serialize, Deserialize};
use crate::chapters::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Episode {
    pub title: String,
    pub subtitle: String,
    pub season: usize,
    pub number: usize,
    pub author: String,

    pub podcast_url: String,
    pub filename: String,
    pub date_time: String,
    pub duration: String,
    pub explicit: bool,

    pub thumbnail: String,
    pub chapters: Vec<Chapter>,

    pub description: String,
}

// pub to_rss(episode: Episode) -> String {
//     let 

//     format!(
// r#"
// <item>
// <title>$(data[:title])</title>
// <itunes:title>$(data[:title])</itunes:title>
// <itunes:author>$(data[:author])</itunes:author>
// <itunes:subtitle>
// $(data[:subtitle])
// </itunes:subtitle>
// <itunes:summary>
// <![CDATA[
// $(data[:description])
// $(chapters_description)
// <h3>Contact</h3>
// Reach out through email or boostagram, and we will read and discuss it on the show.
// Email: <a href="mailto:counterplan@proton.me">counterplan@proton.me</a>

// <h3>Value4Value</h3>
// If you got some value from the show, please consider giving back through a boost.
// - Install a <a href="newpodcastapps.com">new podcast app</a> and boost in on the episode page.
// ...or, if you want to keep your current podcast app
// - Boost in directly from the <a href="podcastindex.org">Podcast Index</a> 
//   • Install <a href="https://getalby.com/">Alby</a></li>
//   • Find <a href="https://podcastindex.org/podcast/6325209">Counterplan on the Podcast Index</a>
//   • Boost right from the page!

// Acknowledgments
// Art of " <a href="https://thenounproject.com/icon/the-thinker-57467/">The Thinker</a> ", by <a href="https://thenounproject.com/nuno.lezon/">Nuno Lezon</a> from <a href="https://thenounproject.com/">Noun Project</a> under <a href="https://creativecommons.org/licenses/by/3.0/">CC BY 3.0</a>.
// Music by <a href="https://pixabay.com/users/playsound-24686998/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=music&amp;utm_content=11755">Playsound</a> from <a href="https://pixabay.com/music//?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=music&amp;utm_content=11755">Pixabay</a>.
// ]]>
// </itunes:summary>
// <description>
// <![CDATA[
// $(data[:description])
// $(chapters_description)
// <h3>Contact</h3>
// Reach out through email or boostagram, and we will read and discuss it on the show.
// Email: <a href="mailto:counterplan@proton.me">counterplan@proton.me</a>

// <h3>Value4Value</h3>
// If you got some value from the show, please consider giving back through a boost.
// - Install a <a href="newpodcastapps.com">new podcast app</a> and boost in on the episode page.
// ...or, if you want to keep your current podcast app
// - Boost in directly from the <a href="podcastindex.org">Podcast Index</a> 
//   • Install <a href="https://getalby.com/">Alby</a></li>
//   • Find <a href="https://podcastindex.org/podcast/6325209">Counterplan on the Podcast Index</a>
//   • Boost right from the page!

// Acknowledgments
// Art of " <a href="https://thenounproject.com/icon/the-thinker-57467/">The Thinker</a> ", by <a href="https://thenounproject.com/nuno.lezon/">Nuno Lezon</a> from <a href="https://thenounproject.com/">Noun Project</a> under <a href="https://creativecommons.org/licenses/by/3.0/">CC BY 3.0</a>.
// Music by <a href="https://pixabay.com/users/playsound-24686998/?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=music&amp;utm_content=11755">Playsound</a> from <a href="https://pixabay.com/music//?utm_source=link-attribution&amp;utm_medium=referral&amp;utm_campaign=music&amp;utm_content=11755">Pixabay</a>.
// ]]>
// </description>
// <enclosure url="$(data[:podcast_url])/audio/$(formatted_filename)" length="$(mp3_size)" type="audio/mpeg"/>
// <itunes:duration>$(data[:duration])</itunes:duration>
// <itunes:season>$(data[:season])</itunes:season>
// <itunes:episode>$(data[:number])</itunes:episode>
// <itunes:episodeType>full</itunes:episodeType>
// <guid isPermaLink="false">
// "$(data[:podcast_url])/audio/$(formatted_filename)"
// </guid>
// <pubDate>$(data[:date_time])</pubDate>
// <itunes:explicit>$(data[:explicit])</itunes:explicit>
// $(chapters_line)
// $(custom_thumbnail_line)
// </item>
// "#
//         )
// }
