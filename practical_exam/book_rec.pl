/*
    Name: Shashank Attarde
    PRN / Roll No / Seat No: 0120180337 / 015 / T204054
    Branch: TY B.Tech Comp

    Problem Statement: Write a program in prolog for Expert System of Book Recommendation.
*/

/* Specify the various genres by their names */
genre("f", f, fantasy).
genre("a", a, adventure).
genre("k", k, kids).
genre("t", t, thriller).
genre("r", r, romance).
genre("d", d, tragedy).
genre("m", m, mystery).

/* Specify various books with a list of their genres */
book("The Lion, The Witch and the Wardrobe - C.S. Lewis", [f, k]).
book("Harry Potter - J.K. Rowling", [f, a]).
book("The Adventures of Pinocchio - Carlo Collodi", [f, k]).
book("The Da Vinci Code - Dan Brown", [m, t]).
book("Angels and Demons - Dan Brown", [m, t]).
book("Inferno - Dan Brown", [m, t]).
book("Heidi - Johanna Spyri", [k, a]).
book("Killing Floor - Lee Child", [m, t]).
book("The Fault in our Stars - John Green", [d, r]).
book("Looking for Alaska - John Green", [d, r]).
book("Eragon - Christopher Paolini", [f, a]).
book("Eldest - Christopher Paolini", [f, a]).
book("Brisingr - Christopher Paolini", [f, a]).
book("Inheritance - Christopher Paolini", [f, a]).
book("A Game of Thrones - George R.R. Martin", [f, t]).
book("A Clash of Kings - George R.R. Martin", [f, t]).
book("A Storm of Swords - George R.R. Martin", [f, t]).
book("A feast for Crows - George R.R. Martin", [f, t]).
book("A Dance with Dragons - George R.R. Martin", [f, t]).
book("Romeo and Juliet - William Shakespeare", [r, d]).
book("Five Children and It - E. Nesbit", [k, t]).
book("Fifty Shades of Grey - E. L. James", [r, t]).
book("Percy Jackson and the Lightning Thief - Rick Riordan", [m, a]).
book("Percy Jackson and the Sea of Monsters - Rick Riordan", [m, a]).
book("Percy Jackson and the Titan's Curse - Rick Riordan", [m, a]).
book("Percy Jackson and the Battle of The Labyrinth - Rick Riordan", [m, a]).
book("Percy Jackson and the Last Olympian - Rick Riordan", [m, a]).

/* Predicate to fetch book names by genre subset */
book_name_by_genres(Genres, Name) :-
    book(Name, Gs),
    subset(Genres, Gs).

/* Predicate to print book names. Uses the 'forall' predicate which makes it so
   every possibility is printed without the user having to press semicolon */
print_books(Genres) :-
    forall(book_name_by_genres(Genres, Name), writeln(Name)).

/* Get book recommendations by single genre and print them */
print_single_rec(Genre) :-
    genre(_, Genre, Name),
    atomics_to_string(["Recommendations since you chose ", Name], Str),
    writeln(Str),
    print_books([Genre]).

/* Get book recommendations with two genres provided and print them out */
print_double_rec(G1, G2) :-
    genre(_, G1, N1),
    genre(_, G2, N2),
    atomics_to_string(["Recommendations since you chose ", N1, " and ", N2], Str),
    writeln(Str),
    print_books([G1, G2]).

/* Predicate to print allowed genre types at the start */
print_genre(Short, Long) :-
    atomics_to_string([Short, " - ", Long], Str),
    writeln(Str).

main :-
    writeln("Welcome to the Book Recommender System. You Can choose among the following genres"),
    forall(genre(Short, _, Long), print_genre(Short, Long)), nl,
    write("Choose your first Genre: "), read(Genre1), nl,
    write("Do you want to select another Genre for better recommendations? (Enter 'yes'/'no'): "), read(Conf), nl,
    (Conf == yes ->
        write("Choose your second Genre: "), read(Genre2), nl,
        print_double_rec(Genre1, Genre2);
        print_single_rec(Genre1)).

/*
    OUTPUT 1 :-
2 ?- main.
Welcome to the Book Recommender System. You Can choose among the following genres
f - fantasy
a - adventure
k - kids
t - thriller
r - romance
d - tragedy
m - mystery

Choose your first Genre: f.

Do you want to select another Genre for better recommendations? (Enter 'yes'/'no'): |: yes.

Choose your second Genre: |: a.

Recommendations since you chose fantasy and adventure
Harry Potter - J.K. Rowling
Eragon - Christopher Paolini
Eldest - Christopher Paolini
Brisingr - Christopher Paolini
Inheritance - Christopher Paolini
true.


    OUTPUT 2 :-
3 ?- main.
Welcome to the Book Recommender System. You Can choose among the following genres
f - fantasy
a - adventure
k - kids
t - thriller
r - romance
d - tragedy
m - mystery

Choose your first Genre: m.

Do you want to select another Genre for better recommendations? (Enter 'yes'/'no'): |: yes.

Choose your second Genre: |: t.

Recommendations since you chose mystery and thriller
The Da Vinci Code - Dan Brown
Angels and Demons - Dan Brown
Inferno - Dan Brown
Killing Floor - Lee Child
true.

    
    OUTPUT 3 :-
3 ?- main.
Welcome to the Book Recommender System. You Can choose among the following genres
f - fantasy
a - adventure
k - kids
t - thriller
r - romance
d - tragedy
m - mystery

Choose your first Genre: k.

Do you want to select another Genre for better recommendations? (Enter 'yes'/'no'): |: no.

Recommendations since you chose kids
The Adventures of Pinocchio - Carlo Collodi
Heidi - Johanna Spyri
Five Children and It - E. Nesbit
true.
*/