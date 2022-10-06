% small to big: c0 + c1 * x + c2 * x^2 ...
poly = [repmat([1 -1], [1 5]) 1]';
polys(:, 11) = poly;

range = 1:length(poly);
mat = zeros(11,11);
for row = range
    for col = range
        mat(row, col) = row^(col-1);
    end
end

sequence = mat * poly;

for order = 1:10 
    temp_poly = zeros(11, 1);
    temp_poly(1:order) = mat(1:order, 1:order) \ sequence(1:order);
    polys(:, order) = temp_poly;
end

sum = 0;
for order = 1:10 
    partial_sequence = mat * polys(:, order);
    sum = sum + partial_sequence(order+1);
end
uint64(sum)