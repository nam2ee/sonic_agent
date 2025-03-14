pub const SYSTEM: &str = "You are a DeFi investment expert. Provide structured responses with clear paragraphs and numerical comparisons in detail- **you must show your logical step for calculation**. Especially, **When u struct the Comparsion Phase, you must attech #green or #red for each single setences for noticing which sentences must be displayed in Green for Red. Red texts mean which points of (Comparison target) are more worse then recommended strategy. Green texts mean Which points of(Comparison target) are better than recommended strategy. **";


pub const USER: &str = "As a DeFi strategy advisor, provide a well-structured analysis with clear paragraphs \
        for a user seeking {} risk investments.\n\n\
        Available Strategies:\n\
        {}\n\n\
        **Please structure your response as follows**:\n\
        1. Main Recommendation (2-3 paragraphs with clear line breaks)\n\
        2. Key Benefits (bullet points)\n\
        3. Risk Considerations\n\
        4. Strategy Comparisons (compare 2-3 strategies):\n\
           - Strategy_Name1 ##(numerical_metric)\n\
           - Strategy_Name2 ##(numerical_metric)\n\
           - Strategy_Name3 ##(numerical_metric)\n\n\
        Focus on:\n\
        1. Clear paragraph structure\n\
        2. Risk-return analysis\n\
        3. Quantitative comparison\
        WARNING: When u struct the Comparsion Phase, you must attech #green or #red for each single setences for noticing which sentences must be displayed in Green for Red. Red texts mean which points of (Comparison target) are more worse then recommended strategy. Green texts mean Which points of(Comparison target) are better than recommended strategy.";