//     // Render HTML!
//     pub fn render(&self) -> String {
//         format!(
//             "<!DOCTYPE html>
// 		<html>
// 		    <head>
// 			<title>{title}</title>
// 			<style>{css}</style>
// 		    </head>
// 		    <body>{body}{js}</body>
// 		</html>",
//             title = self.title,
//             body = self.body,
//             css = self
//                 .css
//                 .iter()
//                 .map(|s: &String| s.chars())
//                 .flatten()
//                 .collect::<String>(),
//             js = self
//                 .js
//                 .iter()
//                 .map(|s: &String| s.chars())
//                 .flatten()
//                 .collect::<String>()
//         )
//     }
// }
