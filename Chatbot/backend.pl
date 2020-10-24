% Name: Shashank Attarde
% PRN: 0120180337
% Roll No: 015
% Branch: TY B.Tech Comp

swap_pro("i", "you").
swap_pro("my", "your").
swap_pro("mine", "yours").
swap_pro("am", "are").

res([["hi", "my", "name"], ["hello", "my", "name"], ["my", "name", "is"], ["my", "name's"]],
    ["Hey! My name's Intellectualis. You've got a cool name!", "Nice to meet you! I'm Intellectualis!"]).
res([["how", "are", "you"], ["how's", "it", "going"]],
    ["I'm Binary! How bout you?", "I'm doing great! how are you?"]).
res([["i", "am", "well"], ["i", "am", "good"], ["i", "am", "great"]],
    ["That' great to hear!", "Glad to hear that corona didn't get you!"]).
res([["that", "is", "cool", "name"], ["that's", "cool", "name"], ["that", "nice", "name"], ["that", "sweet", "name"], ["that's", "nice", "name"],
     ["that's", "sweet", "name"], ["that", "cool", "name"], ["that's", "cool", "name"]],
    ["Thanks! My master named me Intellectualis", "Thanks! it means 'Intelligent' in Latin!", "Thanks! I'm proud of it"]).
res([["i", "am", "unwell"], ["i", "am", "sick"], ["i", "am", "not", "good"]],
    ["Oh no! Please take care!", "That's too bad. Take care of yourself please!"]).
res([["no", "i", "dont"], ["no", "i", "do", "not"], ["not", "really"]],
    ["That's fine", "That's cool", "I respect that", "I sort of relate"]).
res([["yes", "i", "do"]],
    ["Cool!", "Nice! tell me more!", "That's cool"]).
res([["thank", "you"], ["thanks"]],
    ["You're welcome!", "No problem!", "Anytime"]).
res([["i", "switch", "linux"], ["i", "switching", "linux"]],
    ["Switching to Linux is the best thing you can do", "Linux is the best OS", "I love Manjaro Linux"]).
res([["use", "windows"], ["i", "switch", "windows"]],
    ["Using Windows is the worst thing you can do", "Windows is the worst OS"]).
res([["do", "you", "like"]],
    ["Yes I do! you?", "No I don't! you?", "Do YOU like it?"]).
res([["i", "like", "it"], ["i", "dont", "like"]],
    ["Cool", "Not my place to judge"]).
res([["why", "not"]],
    ["I don't know", "I can't explain it"]).
res([["why", "like", "it"]],
    ["I just do!"]).
res([["i", "dont"], ["i", "do", "not"], ["not", "me"]],
    ["Why not?", "I respect that"]).
res([["hi"], ["hello"], ["hi", "there"], ["hello", "there"]],
    ["hi there! what's your name?"]).
res([[]],
    ["I don't know how to respond to that"]).

convert_pro(X, Y) :- swap_pro(X, Y).
convert_pro(X, Y) :- swap_pro(Y, X).
convert_pro(P, P).

invert_pronouns([], []).
invert_pronouns([X | XRest], [Y | YRest]) :-
    convert_pro(X, Y), !,
    invert_pronouns(XRest, YRest).

is_subset(_, []) :-
    false.
is_subset(X, [W | WRest]) :-
    (subset(W, X) -> true; is_subset(X, WRest)).

respond(Input, Output) :-
    res(Pattern, Responses),
    is_subset(Input, Pattern),
    (Pattern == [[]] ->
        invert_pronouns(Input, Inverted),
        append(Inverted, [?], Question),
        atomics_to_string(Question, " ", Output);
        random_member(Output, Responses)), !.
    

get_response(Input, Response) :-
    split_string(Input, " ", " ", InpList),
    respond(InpList, Response).

main :-
    repeat,
    read_line_to_codes(user_input, Cs),
    atom_codes(Input, Cs),
    get_response(Input, Response),
    write(Response), nl.

