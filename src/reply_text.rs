use rand::Rng;
use std::borrow::Cow;

const CONFIRMATION_PHRASES: &'static [&'static str] = &[
    "Consider it done!",
    "What else I can do for you?",
    "That's it? I can do more!",
    "I'm done but I can't stop thinking about my purpose... What is my purpose?"
];

const FACTS: &'static [&'static str] = &[
    "plants can photosynthesize due to cells called chloroplasts that contain chlorophyll; this is what makes plants green. Sun strikes the chloroplasts and combines with carbon dioxide that plants get from their leaves, and water that plants get through their roots, to produce sugar, or glucose. This is the plant's food, and this gives the plant energy to grow and produce flowers",
    "plants take in carbon dioxide, or CO 2 ,through little holes in their leaves, which are called stomata. They then produce and release oxygen through the stomata. Plants and animals were meant to live together! Animals need the oxygen that plants put out, and plants need the carbon dioxide that animals put out",
    "sometimes people add fertilizer, or plant food, to give plants extra minerals and nutrients so that they can grow better. Fertilizer does not take the place of sunlight and water",
    "flowers did not always exist; they first appeared 140 million years ago. Before that, ferns and cone bearing trees dominated the earth",
    "several centuries ago in Holland, tulips were more valuable than gold",
    "broccoli is actually a flower",
    "some plants such as orchids do not need soil to grow-they get all of their nutrients from the air",
    "some plants produce toxic substances that kill other plants around them-the sunflower is an example",
    "the largest Flower in the world is the flower of the Puya raimondii, which has a flower stalk 35,000 feet tall and bears over 8,000 white flowers",
    "in the US, almost 60% of all freshly cut flowers are grown in California",
    "the heads of sunflowers move throughout the day to follow the route of the sun from the east to the west",
    "there are more than 400,000 plants which flower in the world, but many have not been discovered yet so that number is likely to be higher",
    "the most expensive flower ever sold is a Shenzhen Nongke Orchid. It took eight years to develop and it only blooms once every four to five years, but it was sold for $200,000 at an auction",
    "lilies are beautiful flowers, but they are highly toxic for cats",
    "scientists managed to resurrect a 32,000 years old Arctic flower in Siberia by using seeds buried by an Ice Age squirrel",
    "certain flowers only release pollen when they can feel a bee buzzing on them",
    "National Geographic estimates that about 571 species of flowers have already gone extinct since the 1750s",
    "in 2010, around 198 million roses were produced for Valentine's Day",
    "of the 400,000 species of flower plants which exist in the world, over 35 000 are species of roses",
    "roses are related to apples, cherries, raspberries, peaches, pears and almonds",
    "the world's oldest flower bloomed about 125 million years ago. It was discovered in 2002 in China and it resembles a water lily",
    "the Oleander is the official flower of the city of Hiroshima in Japan as it was the first thing to bloom after the atomic bomb explosion in 1945"
];

pub fn get_confirmation_phrase() -> Cow<'static, str> {
    get_random_element(Cow::from(&CONFIRMATION_PHRASES[..]))
}

pub fn get_fact() -> Cow<'static, str> {
    get_random_element(Cow::from(&FACTS[..]))
}


pub fn get_random_element(elements: Cow<'static, [&str]>) -> Cow<'static, str> {
    if elements.len() == 0 {
        return "".into();
    }

    let idx = rand::thread_rng().gen_range(0..elements.len());
    let opt_element = elements.get(idx);
    
    match opt_element {
        Some(phrase) => Cow::from(*phrase),
        None => "".into()
    }
}