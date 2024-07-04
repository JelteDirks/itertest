loop1 = results.VarName1;
loop2 = results.VarName2;

all = [loop1 loop2];

minval = min(all(:));
maxval = max(all(:));

figure();

subplot(1,2,1);
boxplot(loop1);
xticklabels("loop1");
ylabel("nanoseconds");
ylim([minval, maxval]);


subplot(1,2,2);
boxplot(loop2);
xticklabels("loop2");
ylabel("nanoseconds");
ylim([minval, maxval]);

disp("loop1")
infome(loop1);

disp("loop2")
infome(loop2);

function infome(data)
disp("mean: " + mean(data));
disp("std: " + std(data));
disp("median: " + median(data));
disp("min: " + min(data));
disp("max: " + max(data));
end

