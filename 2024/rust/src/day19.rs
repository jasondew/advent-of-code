use std::collections::{HashMap, VecDeque};

type Pattern = String;
type Design = String;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (all_patterns, designs) = parse(input);
    let available_patterns = all_patterns
        .into_iter()
        .filter(|pattern| designs.iter().any(|design| design.contains(pattern)))
        .collect();

    let mut cache: HashMap<String, bool> = HashMap::new();
    let mut count: usize = 0;

    for design in designs {
        if create(design, &available_patterns, &mut cache).is_some() {
            count += 1;
        }
    }

    count
}

fn create(
    design: Design,
    available_patterns: &Vec<Pattern>,
    cache: &mut HashMap<String, bool>,
) -> Option<VecDeque<Pattern>> {
    if design.is_empty() {
        return Some(VecDeque::new());
    }

    if let Some(exists) = cache.get(&design) {
        if !exists {
            return None;
        }
    }

    for pattern in available_patterns {
        if design.starts_with(pattern) {
            if let Some(mut answer) = create(
                design[pattern.len()..].into(),
                available_patterns,
                cache,
            ) {
                answer.push_front(pattern.clone());
                return Some(answer);
            }

            cache.insert(design.clone(), false);
        }
    }

    None
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (all_patterns, designs) = parse(input);

    let available_patterns: Vec<Pattern> = all_patterns
        .into_iter()
        .filter(|pattern| designs.iter().any(|design| design.contains(pattern)))
        .collect();

    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut count: usize = 0;

    for design in designs {
        count += arrangement_count(design, &available_patterns, &mut cache);
    }

    count
}

fn arrangement_count(
    design: Design,
    available_patterns: &Vec<Pattern>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = cache.get(&design) {
        return *count;
    }

    // xyz
    //  x => yz
    //      y => z
    //          z => 1
    //      yz => 1
    //   xy => z
    //      z => 1

    available_patterns
        .iter()
        .map(|pattern| {
            if design.starts_with(pattern) {
                let rest_of_design: String = design[pattern.len()..].into();
                let count = arrangement_count(
                    rest_of_design.clone(),
                    available_patterns,
                    cache,
                );
                cache.insert(rest_of_design, count);
                count
            } else {
                0
            }
        })
        .sum()
}

fn parse(input: &str) -> (Vec<Pattern>, Vec<Design>) {
    let (pattern_strings, design_strings) =
        input.trim_end().split_once("\n\n").unwrap();
    let mut patterns: Vec<Pattern> =
        pattern_strings.split(", ").map(|s| s.into()).collect();
    let designs = design_strings.lines().map(|s| s.into()).collect();

    patterns.sort_by_key(|p| std::cmp::Reverse(p.len()));

    (patterns, designs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb\n"
    }

    fn v(v: Vec<&str>) -> VecDeque<String> {
        v.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn part1_example() {
        let (patterns, _) = parse(input());

        // brwrr can be made with a br towel, then a wr towel, and then finally an r towel.
        assert_eq!(
            create("brwrr".into(), &patterns, &mut HashMap::new()),
            Some(v(vec!["br", "wr", "r"]))
        );

        // bggr can be made with a b towel, two g towels, and then an r towel.
        assert_eq!(
            create("bggr".into(), &patterns, &mut HashMap::new()),
            Some(v(vec!["b", "g", "g", "r"]))
        );

        // gbbr can be made with a gb towel and then a br towel.
        assert_eq!(
            create("gbbr".into(), &patterns, &mut HashMap::new()),
            Some(v(vec!["gb", "br"]))
        );

        // rrbgbr can be made with r, rb, g, and br.
        assert_eq!(
            create("rrbgbr".into(), &patterns, &mut HashMap::new()),
            Some(v(vec!["r", "rb", "gb", "r"]))
        );

        // ubwu is impossible.
        assert_eq!(create("ubwu".into(), &patterns, &mut HashMap::new()), None);

        // bwurrg can be made with bwu, r, r, and g.
        assert_eq!(
            create("bwurrg".into(), &patterns, &mut HashMap::new()),
            Some(v(vec!["bwu", "r", "r", "g"]))
        );

        // brgr can be made with br, g, and r.
        assert_eq!(
            create("brgr".into(), &patterns, &mut HashMap::new()),
            Some(v(vec!["br", "g", "r"]))
        );

        // bbrgwb is impossible.
        assert_eq!(
            create("bbrgwb".into(), &patterns, &mut HashMap::new()),
            None
        );

        assert_eq!(part1(input()), 6);

        let (patterns, _) = parse("\
gwrgb, rwwr, rrgwbgg, rwgw, wwgu, rrwuww, wgrw, wbgw, gwrg, ugbr, uuru, bgu, brrur, wggbrb, rurrggw, rggb, wgww, bu, ubwug, bruuu, u, ggbrwgb, burgbub, wrbwguuu, wgb, burugbu, wwgr, uwb, grr, ugu, ggbuu, gbrb, guw, buuguu, rrbrrg, wbgurw, wurr, rgbr, uubu, gwr, gbrwu, ugrru, ugbur, urrrwgw, wrbgrw, rruwr, uwwurwu, wbgbbrb, rbwugu, wrr, wub, rggwg, rgw, ugwgbbr, gggg, uwuuwbww, wwwgw, uub, ugurwwgw, ubrg, rwww, grgu, uwu, gbu, gru, rwrwb, wrw, ubugrb, ruu, grw, wbrggg, wbwwr, wruburw, wurb, wwg, bwburg, gwrbg, guwgr, gwu, wwwuw, wwbrw, gruw, uwbggr, uww, gwb, wurrgg, uuww, rwgwgr, wubg, uuur, uwrrrwu, bbww, wgbwb, rrgg, grggwr, bb, ug, wbwr, rw, rbbru, bwbbub, bur, ggbg, gww, buruggrr, bgbw, rru, rur, brbbr, urug, ggrburu, rgb, bwrwrubu, uuugw, uwgwwb, wgu, wguwuww, ruw, gub, bbwb, ubbgrr, ggr, rug, brg, rwg, ubgwrgr, bwwbw, rww, bgruwuwu, uuw, guwr, urg, urr, gggbwu, brub, gruwub, gbbr, uru, wwur, grrrwbg, rr, guwwuw, ruwr, bubw, bgru, bbggb, wuwbrrg, w, rgr, rguu, wugr, rrbubrgg, wugrr, bru, ggguubr, brwgw, gwbbgbgr, bbu, brr, rubgu, burgrr, wwgw, rwu, rrgubgu, uwr, bguwbr, gw, gur, ruurb, wbur, urub, wrwbubg, ur, uggwr, bbbb, rrg, rgwg, rguw, rbwgr, uugb, grguwwgr, bgubw, bbg, rgbw, wgwrubrr, gwug, ubrugwr, uuwgur, bwurw, ggu, gubb, gbbub, gwg, gugw, r, wgg, wwbb, buubg, rgbwubug, urbb, bg, wwrur, rwr, ugug, bwg, rbb, bbwgru, gbr, urgwb, rbuwub, gbb, rwwgw, grb, wrbw, rrbbwr, rrrubb, rrggr, uugug, rwbr, rguwuuw, wwu, wruggur, wurbb, urrg, urgwrrr, uuu, wrggbg, www, rwrgg, wbguru, bwgr, wu, bbgr, guu, wbrgurw, rbg, ubgrrg, uggrw, wbbwgwbg, uur, brbrbggb, uwbrr, gwrr, ugw, wg, wrrurrb, uggb, bw, urgwg, wwrbwu, bwwbguw, bwb, brguu, ugrbr, bwrg, bbw, bugwu, gb, gugu, gwwurwu, rbw, wuuw, guug, ubb, wrg, uguw, brw, bgg, wrgr, wbrugb, gwgug, uwuurug, rrgu, brrrwu, wbr, uu, rwrru, uwrgugw, gwugu, gugrg, wgr, ggurb, wrur, wgwrug, bwuw, bbrguu, grru, bwur, grgbgbgr, rggub, ggb, wbb, uubbrw, wubb, rubrwb, gggrg, bwbb, uw, rbgwrg, wbgr, bggbb, rwb, buw, ubr, gg, rurrgu, gr, rrb, ugb, gbrrb, rub, rgubbbg, wuwg, wrgu, wbu, wbwgb, ruwbguu, gbbrgbu, urgbgb, bgr, buu, wbg, uruuw, bwuwugw, wwbrr, wguu, rubru, bbb, urb, bug, wgrbw, bwu, bgur, gu, ubgg, gbbw, uguu, rwrb, wru, bubrwu, wwr, wuwub, bbr, uug, burw, uwg, rurwbbr, wbw, wbwb, wuw, wur, brbwr, wr, wbrwug, ugrgwu, wb, ubu, ggg, brb, wwb, ubg, gug, urbuwg, rgg, b, br, wrrbbw, ggbbg, rrggw, rbu, brbrw, rbwrgb, ubw, wuu, gbwur, gbg, bbbggubb, wguuub, grrbwu, gwwgb, rbwru, ub, gwbub, burgu, wgw, uurbwbw, brrw, rb, brrbbbu, bggg, rugr, ugg, ggw, gwgwr, grurw, bwrgbw, bbgwuuw, rgbwwu, rrrbg, bggu, brwb, bbbgur, ruubrb, wubgrru, brrg, uubbr, ubbrugrw, gubrurwr, rrw, wrb, rgwbw, uubgru, wrwg, bgw, ruuw, gbwrg, ww, rrrbw, gurbrr, ruuug, wrrr, ugr, wubbrwbg, gbwrwrgb, bugr, gruuwg, rrr, bubg, bgbrwr, wug, rgu, buugwb, grubgr, grur, gbw, wwrgb, bgwub, rbr, rrwb, gwuwrb, rwwbbrr, grurbbgr, wrwubb, urbrwug, bbrw, ubbb, bub

unused
");

        assert_eq!(
            create(
                "wuwgbububbrgbbuurbrrrbbrwgbwgbubrubbbubwgwubbwbbrrgrwbrwgrg"
                    .into(),
                &patterns,
                &mut HashMap::new()
            ),
            None
        );
    }

    #[test]
    fn part2_example() {
        let (patterns, _) = parse(input());
        let cache = &mut HashMap::new();

        assert_eq!(arrangement_count("brwrr".into(), &patterns, cache), 2);
        assert_eq!(arrangement_count("bggr".into(), &patterns, cache), 1);
        assert_eq!(arrangement_count("gbbr".into(), &patterns, cache), 4);
        assert_eq!(arrangement_count("rrbgbr".into(), &patterns, cache), 6);
        assert_eq!(arrangement_count("bwurrg".into(), &patterns, cache), 1);
        assert_eq!(arrangement_count("brgr".into(), &patterns, cache), 2);
        assert_eq!(arrangement_count("ubwu".into(), &patterns, cache), 0);
        assert_eq!(arrangement_count("bbrgwb".into(), &patterns, cache), 0);

        let (patterns, _) = parse("\
gwrgb, rwwr, rrgwbgg, rwgw, wwgu, rrwuww, wgrw, wbgw, gwrg, ugbr, uuru, bgu, brrur, wggbrb, rurrggw, rggb, wgww, bu, ubwug, bruuu, u, ggbrwgb, burgbub, wrbwguuu, wgb, burugbu, wwgr, uwb, grr, ugu, ggbuu, gbrb, guw, buuguu, rrbrrg, wbgurw, wurr, rgbr, uubu, gwr, gbrwu, ugrru, ugbur, urrrwgw, wrbgrw, rruwr, uwwurwu, wbgbbrb, rbwugu, wrr, wub, rggwg, rgw, ugwgbbr, gggg, uwuuwbww, wwwgw, uub, ugurwwgw, ubrg, rwww, grgu, uwu, gbu, gru, rwrwb, wrw, ubugrb, ruu, grw, wbrggg, wbwwr, wruburw, wurb, wwg, bwburg, gwrbg, guwgr, gwu, wwwuw, wwbrw, gruw, uwbggr, uww, gwb, wurrgg, uuww, rwgwgr, wubg, uuur, uwrrrwu, bbww, wgbwb, rrgg, grggwr, bb, ug, wbwr, rw, rbbru, bwbbub, bur, ggbg, gww, buruggrr, bgbw, rru, rur, brbbr, urug, ggrburu, rgb, bwrwrubu, uuugw, uwgwwb, wgu, wguwuww, ruw, gub, bbwb, ubbgrr, ggr, rug, brg, rwg, ubgwrgr, bwwbw, rww, bgruwuwu, uuw, guwr, urg, urr, gggbwu, brub, gruwub, gbbr, uru, wwur, grrrwbg, rr, guwwuw, ruwr, bubw, bgru, bbggb, wuwbrrg, w, rgr, rguu, wugr, rrbubrgg, wugrr, bru, ggguubr, brwgw, gwbbgbgr, bbu, brr, rubgu, burgrr, wwgw, rwu, rrgubgu, uwr, bguwbr, gw, gur, ruurb, wbur, urub, wrwbubg, ur, uggwr, bbbb, rrg, rgwg, rguw, rbwgr, uugb, grguwwgr, bgubw, bbg, rgbw, wgwrubrr, gwug, ubrugwr, uuwgur, bwurw, ggu, gubb, gbbub, gwg, gugw, r, wgg, wwbb, buubg, rgbwubug, urbb, bg, wwrur, rwr, ugug, bwg, rbb, bbwgru, gbr, urgwb, rbuwub, gbb, rwwgw, grb, wrbw, rrbbwr, rrrubb, rrggr, uugug, rwbr, rguwuuw, wwu, wruggur, wurbb, urrg, urgwrrr, uuu, wrggbg, www, rwrgg, wbguru, bwgr, wu, bbgr, guu, wbrgurw, rbg, ubgrrg, uggrw, wbbwgwbg, uur, brbrbggb, uwbrr, gwrr, ugw, wg, wrrurrb, uggb, bw, urgwg, wwrbwu, bwwbguw, bwb, brguu, ugrbr, bwrg, bbw, bugwu, gb, gugu, gwwurwu, rbw, wuuw, guug, ubb, wrg, uguw, brw, bgg, wrgr, wbrugb, gwgug, uwuurug, rrgu, brrrwu, wbr, uu, rwrru, uwrgugw, gwugu, gugrg, wgr, ggurb, wrur, wgwrug, bwuw, bbrguu, grru, bwur, grgbgbgr, rggub, ggb, wbb, uubbrw, wubb, rubrwb, gggrg, bwbb, uw, rbgwrg, wbgr, bggbb, rwb, buw, ubr, gg, rurrgu, gr, rrb, ugb, gbrrb, rub, rgubbbg, wuwg, wrgu, wbu, wbwgb, ruwbguu, gbbrgbu, urgbgb, bgr, buu, wbg, uruuw, bwuwugw, wwbrr, wguu, rubru, bbb, urb, bug, wgrbw, bwu, bgur, gu, ubgg, gbbw, uguu, rwrb, wru, bubrwu, wwr, wuwub, bbr, uug, burw, uwg, rurwbbr, wbw, wbwb, wuw, wur, brbwr, wr, wbrwug, ugrgwu, wb, ubu, ggg, brb, wwb, ubg, gug, urbuwg, rgg, b, br, wrrbbw, ggbbg, rrggw, rbu, brbrw, rbwrgb, ubw, wuu, gbwur, gbg, bbbggubb, wguuub, grrbwu, gwwgb, rbwru, ub, gwbub, burgu, wgw, uurbwbw, brrw, rb, brrbbbu, bggg, rugr, ugg, ggw, gwgwr, grurw, bwrgbw, bbgwuuw, rgbwwu, rrrbg, bggu, brwb, bbbgur, ruubrb, wubgrru, brrg, uubbr, ubbrugrw, gubrurwr, rrw, wrb, rgwbw, uubgru, wrwg, bgw, ruuw, gbwrg, ww, rrrbw, gurbrr, ruuug, wrrr, ugr, wubbrwbg, gbwrwrgb, bugr, gruuwg, rrr, bubg, bgbrwr, wug, rgu, buugwb, grubgr, grur, gbw, wwrgb, bgwub, rbr, rrwb, gwuwrb, rwwbbrr, grurbbgr, wrwubb, urbrwug, bbrw, ubbb, bub

unused
");
        assert_eq!(
            arrangement_count(
                "wuwgbububbrgbbuurbrrrbbrwgbwgbubrubbbubwgwubbwbbrrgrwbrwgrg"
                    .into(),
                &patterns,
                cache
            ),
            62809564965852
        );

        assert_eq!(part2(input()), 16);
    }
}
