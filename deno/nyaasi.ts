import fastXmlParser from "https://cdn.skypack.dev/fast-xml-parser"

interface ResultItem {
  link: string
  title: string
  size: string
  seeders: string
  leechers: string
  downloads: string
  infoHash: string
  category: string
}

const escapeSpaces = (str: string) => {
  return str.replace(/\s/g, "+")
}

const formatResult = (data: ResultItem) => {
  return `\x1b[1;37m${"─".repeat(60)} \x1b[0;0m
\x1b[1;33m Title: \x1b[0;0m${data.title}
\x1b[1;33m﩯Category: \x1b[0;0m${data.category}
\x1b[1;33m Link: \x1b[0;0;4;3;37m${data.link}\x1b[0;0m
\x1b[1;33m遲Size: \x1b[0;0m${data.size}
\x1b[1;33mﯲ Leechers: \x1b[0;0m${data.leechers}
\x1b[1;33mﯴ Seeders: \x1b[0;0m${data.seeders}
\x1b[1;33m Downloads: \x1b[0;0m${data.downloads}
\x1b[1;33m Info Hash: \x1b[0;0m${data.infoHash}
`
}

const fetchTorrent = async (query: string) => {
  const NYAA_SI_URL = "https://nyaa.si/?page=rss&q="
  const escapedQuery = escapeSpaces(query)

  console.log(`\x1b[1;34mVisiting ${NYAA_SI_URL}${escapedQuery}...`)

  const response = await fetch(`${NYAA_SI_URL}${escapedQuery}`)
  const xml = await response.text()
  const { rss } = fastXmlParser.parse(xml)

  return rss.channel.item
    .map((item: { [key: string]: string }) => {
      return formatResult({
        title: item["title"],
        category: item["nyaa:category"],
        link: item["link"],
        size: item["nyaa:size"],
        leechers: item["nyaa:leechers"],
        seeders: item["nyaa:seeders"],
        downloads: item["nyaa:downloads"],
        infoHash: item["nyaa:infoHash"],
      })
    })
    .join("")
}

console.log(await fetchTorrent(Deno.args[0]))
